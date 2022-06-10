// Copyright (c) 2022 Riley Seaburg
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::RUDIS_DB;
use resp::Value;

/// The `process_client_request` function is used to process a client request.
/// ## Fuction Name
/// # `process_client_request`
/// ### Description:
/// Process a client request.
///
/// ## Privacy
/// #### Public
///
/// ## Arguments
/// * `decoded_msg` - The decoded message.
/// * - Type: `resp::Value`
/// * - Description: The decoded message.
///
/// ## Return
/// * - Type: `Vec<u8>`
/// * - Description: The response.
///
/// # Examples
///
/// ```
/// use rudis::commands::process_client_request;
/// use resp::Value;
///
/// let decoded_msg = Value::Array(vec![
///    Value::BulkString(b"SET".to_vec()),
///   Value::BulkString(b"key".to_vec()),
///  Value::BulkString(b"value".to_vec()),
/// ]);
///
/// let response = process_client_request(decoded_msg);
/// ```
///
pub fn process_client_request(decoded_msg: Value) -> Vec<u8> {
    // Get the command from the decoded message.
    let reply = if let Value::Array(v) = decoded_msg {
        // Get the first element of the array.
        match &v[0] {
            // If the command is `GET`,
            Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
            // If the command is `SET`,
            Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
            // If the command is `DEL`,
            //      Value::Bulk(ref s) if s == "DEL" || s == "del" => handle_del(v),
            // If any other command is provided,
            other => unimplemented!("{:?} is not supported in this implementation", other),
        }
    } else {
        // If the decoded message is not an array,
        // return an error message.
        Err(Value::Error("ERR invalid command".to_owned()))
    };

    // Convert the reply to a RESP message.
    match reply {
        Ok(r) | Err(r) => r.encode(),
    }
}

/// The `handle_get` function is used to handle a `GET` request.
/// ## Fuction Name
/// # `handle_get`
/// ### Description:
/// Handle a `GET` request.
/// ## Privacy
/// #### Public
/// ## Arguments
/// * `v` - The decoded message.
/// * - Type: `Vec<Value>`
/// * - Description: The decoded message.
/// ## Return
/// * - Type: `Result<Value, Value>`
/// * - Description: The response.
/// # Examples
/// ```
/// use rudis::commands::handle_get;
/// use resp::Value;
/// let v = vec![
///   Value::BulkString(b"GET".to_vec()),
///  Value::BulkString(b"key".to_vec()),
/// ];
///
/// let response = handle_get(v);
/// ```
///
pub fn handle_get(v: Vec<Value>) -> Result<Value, Value> {
    // Declare a value to store the response.
    let v = v.iter().skip(1).collect::<Vec<_>>();
    // If the parameter is empty,
    // return an error message.
    if v.is_empty() {
        return Err(Value::Error(
            "ERR wrong number of arguments for 'GET' command. Expected at least one argument."
                .to_owned(),
        ));
    }
    // Assign a database reference.
    let db_ref = RUDIS_DB.lock().unwrap();
    // Declare a value to store the response.
    let reply = if let Value::Bulk(ref s) = v[0] {
        // If the parameter is a string,
        // return the value associated with the key.
        db_ref
            .get(s)
            .map(|e| Value::Bulk(e.to_string()))
            .unwrap_or(Value::Null)
    } else {
        // If the parameter is not a string,
        // Return NULL.
        Value::Null
    };
    // Return the response.
    Ok(reply)
}

/// The `handle_set` function is used to handle a `SET` request.
/// ## Fuction Name
/// # `handle_set`
/// ### Description:
/// Handle a `SET` request.
/// ## Privacy
/// #### Public
/// ## Arguments
/// * `v` - The decoded message.
/// * - Type: `Vec<Value>`
/// * - Description: The decoded message.
/// ## Return
/// * - Type: `Result<Value, Value>`
/// * - Description: The response.
/// # Examples
/// ```
/// use rudis::commands::handle_set;
/// use resp::Value;
/// let v = vec![
///  Value::BulkString(b"SET".to_vec()),
/// Value::BulkString(b"key".to_vec()),
/// Value::BulkString(b"value".to_vec()),
/// ];
/// let response = handle_set(v);
/// ```

pub fn handle_set(v: Vec<Value>) -> Result<Value, Value> {
    // Declare a value to store the response.
    let v = v.iter().skip(1).collect::<Vec<_>>();
    // If the parameter is empty, or the parameters are less than two,
    // return an error message.
    if v.is_empty() || v.len() < 2 {
        return Err(Value::Error(
            "ERR wrong number of arguments for 'SET' command. Expected at least two arguments."
                .to_owned(),
        ));
    }
    // Match the parameter.
    match (&v[0], &v[1]) {
        (Value::Bulk(k), Value::Bulk(v)) => {
            // Create a database reference.
            let _ = RUDIS_DB
                .lock()
                .unwrap()
                .insert(k.to_string(), v.to_string());
        }
        _ => unimplemented!("SET not implemented for {:?}", v),
        // Return an ok message.
    }
    Ok(Value::String("OK".to_string()))
}

pub fn delete_set(v: Vec<Value>) -> Result<Value, Value> {
    // Declare a value to store the response.
    let v = v.iter().skip(1).collect::<Vec<_>>();
    // If the parameter is empty, or the parameters are less than two,
    // return an error message.
    if v.is_empty() || v.len() < 2 {
        return Err(Value::Error(
            "ERR wrong number of arguments for 'SET' command. Expected at least two arguments."
                .to_owned(),
        ));
    }
    // Match the parameter.
    match (&v[0], &v[1]) {
        (Value::Bulk(k), Value::Bulk(v)) => {
            // Create a database reference.
            let _ = RUDIS_DB
                .lock()
                .unwrap()
                .remove(k);
        }
        _ => unimplemented!("SET not implemented for {:?}", v),
        // Return an ok message.
    }
    Ok(Value::String("OK".to_string()))
}
