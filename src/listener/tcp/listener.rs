/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::io::*;
use std::net::{TcpListener, TcpStream};

use crate::cli::args::CLi;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read_exact(&mut buffer).unwrap();

    let received_data = String::from_utf8_lossy(&buffer);
    log::error!("Received data from client: {}", received_data);

    stream.write_all("Hello from server!".as_bytes()).unwrap();
}

pub fn listener(cli: CLi) {
    let listener = TcpListener::bind(cli.address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                log::error!("Error while accepting connection: {}", e);
            }
        }
    }
}
