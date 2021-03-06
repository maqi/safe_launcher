// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use maidsafe_utilities::thread::RaiiThreadJoiner;

const SECURE_COMM_THREAD_NAME: &'static str = "SecureCommunicationThread";

pub struct SecureCommunication {
    observer         : ::launcher::ipc_server::ipc_session::EventSenderToSession<::launcher
                                                                                 ::ipc_server
                                                                                 ::ipc_session
                                                                                 ::events::SecureCommunicationEvent>,
    symm_key         : ::sodiumoxide::crypto::secretbox::Key,
    symm_nonce       : ::sodiumoxide::crypto::secretbox::Nonce,
    ipc_stream       : ::launcher::ipc_server::ipc_session::stream::IpcStream,
    parser_parameters: ::launcher::parser::ParameterPacket,
}

impl SecureCommunication {
    pub fn new(client           : ::std::sync::Arc<::std::sync::Mutex<::safe_core::client::Client>>,
               observer         : ::launcher::ipc_server::ipc_session::EventSenderToSession<::launcher
                                                                                            ::ipc_server
                                                                                            ::ipc_session
                                                                                            ::events::SecureCommunicationEvent>,
               symm_key         : ::sodiumoxide::crypto::secretbox::Key,
               symm_nonce       : ::sodiumoxide::crypto::secretbox::Nonce,
               ipc_stream       : ::launcher::ipc_server::ipc_session::stream::IpcStream,
               app_root_dir_key : ::safe_nfs::metadata::directory_key::DirectoryKey,
               safe_drive_access: ::std::sync::Arc<::std::sync::Mutex<bool>>) -> RaiiThreadJoiner {
        let joiner = thread!(SECURE_COMM_THREAD_NAME, move || {
            let safe_drive_dir_key = {
                let dir_helper = ::safe_nfs::helper::directory_helper::DirectoryHelper::new(client.clone());
                let user_root_dir_listing = eval_send_one!(dir_helper.get_user_root_directory_listing(), &observer);
                eval_send_one!(user_root_dir_listing.find_sub_directory(&::config::SAFE_DRIVE_DIR_NAME.to_string())
                                                    .ok_or(::errors::LauncherError::from("Could not find SAFEDrive")),
                               &observer).get_key().clone()
            };

            let parameter_packet = ::launcher::parser::ParameterPacket {
                client            : client,
                app_root_dir_key  : app_root_dir_key,
                safe_drive_access : safe_drive_access,
                safe_drive_dir_key: safe_drive_dir_key,
            };

            let mut secure_comm_obj = SecureCommunication {
                observer         : observer,
                symm_key         : symm_key,
                symm_nonce       : symm_nonce,
                ipc_stream       : ipc_stream,
                parser_parameters: parameter_packet,
            };

            secure_comm_obj.run();

            debug!("Exiting Thread {:?}", SECURE_COMM_THREAD_NAME);
        });

        RaiiThreadJoiner::new(joiner)
    }

    fn run(&mut self) {
        loop {
            let cipher_text = eval_send_one!(self.ipc_stream.read_payload(), &self.observer);

            match self.on_receive_payload(&cipher_text) {
                Ok(parser_response) => {
                    if let Some(response_json_str) = parser_response {
                        match self.get_encrypted_normal_response(&cipher_text, response_json_str) {
                             Ok(response_cipher) => eval_send_one!(self.ipc_stream.write(response_cipher), &self.observer),
                             Err(err) => debug!("{:?} - Failed to construct a normal response for peer.", err),
                         }
                    } else {
                        match self.get_encrypted_error_response(&cipher_text, None) {
                             Ok(response_cipher) => eval_send_one!(self.ipc_stream.write(response_cipher), &self.observer),
                             Err(err) => debug!("{:?} - Failed to construct a response error for peer.", err),
                        }
                    }
                },
                Err(err) => {
                    match self.get_encrypted_error_response(&cipher_text, Some(err)) {
                         Ok(response_cipher) => eval_send_one!(self.ipc_stream.write(response_cipher), &self.observer),
                         Err(err) => debug!("{:?} - Failed to construct a response error for peer.", err),
                    }
                },
            }
        }
    }

    fn on_receive_payload(&self, cipher_text: &[u8]) -> ::launcher::parser::ResponseType {
        let plain_text = try!(::sodiumoxide::crypto::secretbox::open(&cipher_text, &self.symm_nonce, &self.symm_key)
                                                               .map_err(|()| ::errors::LauncherError::SymmetricDecipherFailure));
        let json_str = try!(parse_result!(String::from_utf8(plain_text), "Invalid UTF-8"));
        let json_request = try!(::rustc_serialize::json::Json::from_str(&json_str));

        ::launcher::parser::begin_parse(self.parser_parameters.clone(), &mut ::rustc_serialize::json::Decoder::new(json_request))
    }

    fn get_encrypted_normal_response(&self,
                                     orig_payload: &[u8],
                                     data        : String) -> Result<Vec<u8>, ::errors::LauncherError> {
        let normal_response = LauncherNormalResponse {
            id  : SecureCommunication::get_response_id(orig_payload),
            // TODO(Spandan)
            // This is inefficient - encoding into a json_str and then again decoding that into a
            // JSON. Instead get directly into a JSON in ::launcher::parser::ResponseType
            data: unwrap_result!(::rustc_serialize::json::Json::from_str(&data[..])),
        };

        let json_str = try!(::rustc_serialize::json::encode(&normal_response));

        let cipher_text = ::sodiumoxide::crypto::secretbox::seal(&json_str.into_bytes(), &self.symm_nonce, &self.symm_key);

        Ok(cipher_text)
    }

    fn get_encrypted_error_response(&self,
                                    orig_payload: &[u8],
                                    error       : Option<::errors::LauncherError>) -> Result<Vec<u8>, ::errors::LauncherError> {
        let response_id = SecureCommunication::get_response_id(orig_payload);

        let (debug_description, error_code) = if let Some(err) = error {
            (format!("{:?}", err), err.into())
        } else {
            (String::new(), 0i32)
        };

        let error_detail = ErrorDetail {
            code       : error_code as i64,
            description: debug_description,
        };

        let error_response = LauncherErrorResponse {
            id   : response_id,
            error: error_detail,
        };

        let json_str = try!(::rustc_serialize::json::encode(&error_response));

        let cipher_text = ::sodiumoxide::crypto::secretbox::seal(&json_str.into_bytes(), &self.symm_nonce, &self.symm_key);

        Ok(cipher_text)
    }

    fn get_response_id(orig_payload: &[u8]) -> String {
        use rustc_serialize::base64::ToBase64;

        let digest = ::sodiumoxide::crypto::hash::sha512::hash(orig_payload);
        digest.0.to_base64(::config::get_base64_config())
    }
}

#[derive(RustcEncodable, Debug)]
struct LauncherNormalResponse {
    id  : String,
    data: ::rustc_serialize::json::Json,
}

#[derive(RustcEncodable, Debug)]
struct LauncherErrorResponse {
    id   : String,
    error: ErrorDetail,
}

#[derive(RustcEncodable, Debug)]
struct ErrorDetail {
    code       : i64,
    description: String,
}
