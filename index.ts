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

// TYPESCRIPT_NEXT_FUNCTION is the name of the next function in the typescript guest runtime
export const TYPESCRIPT_NEXT_FUNCTION: string = "scale_next";

// TYPESCRIPT_ADDRESS_OF is the name of the address_of function in the typescript guest runtime
export const TYPESCRIPT_ADDRESS_OF: string = "scale_address_of";

// New is a factory function for creating a new Signature
export type New<T extends Signature> = () => T;

// Signature is an interface that must be implemented by all Scale Signatures
// for use with the Host. Guest implementations do not use this interface.
export interface Signature {
    Read(b: Uint8Array): Error | undefined;
    Write(): Uint8Array;
    Error(err: Error): Uint8Array;
    Hash(): string;
}
