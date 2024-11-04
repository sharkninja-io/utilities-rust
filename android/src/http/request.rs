use crate::http::list::AndroidList;
use crate::traits::{JObjectRustBridge, JavaClass};
use crate::{
    java_class_names::CLASSNAMES,
    java_signatures::{BYTE_SIG, INT_SIG, STRING_SIG, VOID_SIG},
    jni_exts::{byte_array::AndroidData, jobject::MantleJObject, string::AndroidString},
};
use ctor::ctor;
use jni::{
    objects::{JObject, JValue},
    sys::jobject,
};
use log::error;
use mantle_utilities::http::request::{Header, Request};
use mantle_utilities::http::response::Response;

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaRequest::full_name(None));
    names.push(JavaHeader::full_name(None));
    names.push(JavaResponse::full_name(None));
}

const JAVA_PACKAGE: &str = "com/sharkninja/api/mantleutilities/httpclient/";

pub const HTTP_REQUEST_SIG: &str = "Lcom/sharkninja/api/mantleutilities/httpclient/Request;";
pub const HTTP_HEADER_SIG: &str = "Lcom/sharkninja/api/mantleutilities/httpclient/Header;";
pub const HTTP_RESPONSE_SIG: &str = "Lcom/sharkninja/api/mantleutilities/httpclient/Response;";

pub struct JavaRequest(pub Request);
impl JavaClass<Request> for JavaRequest {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("Request");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(",
            STRING_SIG,
            STRING_SIG,
            "[",
            BYTE_SIG,
            INT_SIG,
            "[",
            HTTP_HEADER_SIG,
            ")",
            VOID_SIG,
        ]
        .concat()
    }

    fn j_object(&self, jni_env: jni::JNIEnv, j_class: jni::objects::JClass) -> jobject {
        let signature = JavaRequest::signature(None);

        let url = AndroidString(self.0.url.to_owned()).to_jstring(jni_env);
        let method = AndroidString(self.0.method.to_string()).to_jstring(jni_env);
        let body = match &self.0.body {
            None => JValue::from(JObject::null()),
            Some(body) => JValue::from(AndroidData::to_jbyte_array(&body[..], jni_env)),
        };
        let timeout = JValue::Int(self.0.timeout as i32);
        let java_headers: Vec<JavaHeader> =
            self.0.headers.iter().cloned().map(JavaHeader).collect();
        let headers = AndroidList(java_headers).into_jobject(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/api/mantleutilities/httpclient/Request **
        let args = &[
            JValue::from(url.into_inner()),
            JValue::from(method.into_inner()),
            body,
            timeout,
            JValue::from(JObject::from(headers)),
        ];

        let request_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating httpclient.Request for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *request_object
    }

    fn new(rust_object: Request) -> Self {
        Self(rust_object)
    }
}

pub struct JavaHeader(pub Header);
impl JavaClass<Header> for JavaHeader {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("Header");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        ["(", STRING_SIG, STRING_SIG, ")", VOID_SIG].concat()
    }

    fn j_object(&self, jni_env: jni::JNIEnv, j_class: jni::objects::JClass) -> jobject {
        let signature = JavaHeader::signature(None);

        let key = AndroidString(self.0.key.to_owned()).to_jstring(jni_env);
        let value = AndroidString(self.0.value.to_owned()).to_jstring(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/api/mantleutilities/httpclient/Header **
        let args = &[
            JValue::from(key.into_inner()),
            JValue::from(value.into_inner()),
        ];

        let header_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating httpclient.Header for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *header_object
    }

    fn new(rust_object: Header) -> Self {
        Self(rust_object)
    }
}

pub struct JavaResponse(pub Response);
impl JavaClass<Response> for JavaResponse {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("Response");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        ["([", BYTE_SIG, INT_SIG, ")", VOID_SIG].concat()
    }

    fn j_object(&self, jni_env: jni::JNIEnv, j_class: jni::objects::JClass) -> jobject {
        let signature = JavaResponse::signature(None);

        let bytes = AndroidData::to_jbyte_array(&self.0.content[..], jni_env);
        let status_code = JValue::Int(self.0.status_code as i32);

        // ** Order matters!!! Refer to com/sharkninja/api/mantleutilities/httpclient/Header **
        let args = &[JValue::from(JObject::from(bytes)), status_code];

        let response_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating httpclient.Response for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *response_object
    }

    fn new(rust_object: Response) -> Self {
        Self(rust_object)
    }
}

impl JObjectRustBridge<Response> for JavaResponse {
    fn rust_object(j_object: MantleJObject, jni_env: jni::JNIEnv) -> Option<Response> {
        if !j_object.0.is_null() {
            let status_code = (j_object.to_unsigned_int_field(jni_env, "statusCode") as u16)
                .try_into()
                .unwrap();
            let content = j_object.to_byte_array_field(jni_env, "content");
            let response = Response {
                headers: Default::default(),
                content,
                status_code,
            };
            Some(response)
        } else {
            None
        }
    }
}
