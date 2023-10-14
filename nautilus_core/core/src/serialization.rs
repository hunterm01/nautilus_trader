// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2023 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use pyo3::{prelude::*, types::PyDict, Py, PyErr, Python};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::python::to_pyvalue_err;

/// Represents types which are serializable for JSON and `MsgPack` specifications.
pub trait Serializable: Serialize + for<'de> Deserialize<'de> {
    /// Deserialize an object from JSON encoded bytes.
    fn from_json_bytes(data: Vec<u8>) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&data)
    }

    /// Deserialize an object from `MsgPack` encoded bytes.
    fn from_msgpack_bytes(data: Vec<u8>) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(&data)
    }

    /// Serialize an object to JSON encoded bytes.
    fn as_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Serialize an object to `MsgPack` encoded bytes.
    fn as_msgpack_bytes(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec_named(self)
    }
}

#[cfg(feature = "python")]
pub fn from_dict_pyo3<T>(py: Python<'_>, values: Py<PyDict>) -> Result<T, PyErr>
where
    T: DeserializeOwned,
{
    // Extract to JSON string
    let json_str: String = PyModule::import(py, "json")?
        .call_method("dumps", (values,), None)?
        .extract()?;

    // Deserialize to object
    let instance = serde_json::from_slice(&json_str.into_bytes()).map_err(to_pyvalue_err)?;
    Ok(instance)
}
