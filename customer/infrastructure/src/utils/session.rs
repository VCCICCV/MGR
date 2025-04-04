use domain::model::dto::info::SessionKey;
use uuid::Uuid;
//  生成session_key和session_id
pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
  let session_id = Uuid::new_v4();
  let key = SessionKey { user_id };
  (key, session_id)
}
