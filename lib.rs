/*
    Copyright 2022 Loophole Labs

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

           http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

// TYPESCRIPT_NEXT is the name of the next function in the typescript guest runtime
pub const TYPESCRIPT_NEXT: &str = "scale_next";

// TYPESCRIPT_ADDRESS_OF is the name of the address_of function in the typescript guest runtime
pub const TYPESCRIPT_ADDRESS_OF: &str = "scale_address_of";

// New is a factory function for creating a new Signature
#[allow(type_alias_bounds)]
pub type New<T: Signature> = fn() -> T;

// Signature is an interface that must be implemented by all Scale Signatures
// for use with the Host. Guest implementations do not use this interface.
pub trait Signature {
    fn read(&mut self, b: &mut Vec<u8>) -> Option<Box<dyn std::error::Error>>;
    fn write(&self) -> Vec<u8>;
    fn error(&self, err: Box<dyn std::error::Error>) -> Vec<u8>;
    fn hash(&self) -> String;
}
