/*
	Copyright 2023 Loophole Labs

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

package interfaces

// TYPESCRIPT_NEXT is the name of the next function in the typescript guest runtime
const TYPESCRIPT_NEXT = "scale_next" //nolint:revive

// TYPESCRIPT_ADDRESS_OF is the name of the address_of function in the typescript guest runtime
const TYPESCRIPT_ADDRESS_OF = "scale_address_of" //nolint:revive

// New is a factory function for creating a new Signature
type New[T Signature] func() T

// Signature is an interface that must be implemented by all Scale Signatures
// for use with the Host. Guest implementations do not use this interface.
type Signature interface {
	Read(b []byte) error    // Read reads the signature from the given byte slice
	Write() []byte          // Write writes the signature into a byte slice and returns it
	Error(err error) []byte // Error writes the error into a byte slice and returns it
	Hash() string           // Hash returns the SHA256 hash of the Signature
}
