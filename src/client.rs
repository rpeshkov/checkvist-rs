use crate::models::Checklist;

pub struct Client {
    auth: String,
}

impl Client {
    pub fn new(login: String, api_key: String) -> Self {
        let credentials = base64::encode(format!("{}:{}", &login, &api_key));
        let auth = format!("Basic {}", credentials);

        Self {
            auth
        }
    }

    pub fn get_checklists(&self) -> Result<Vec<Checklist>, ureq::Error> {
        let checklists: Vec<Checklist> = ureq::get("https://checkvist.com/checklists.json")
            .set("Authorization", &self.auth)
            .call()?
            .into_json()?;
        Ok(checklists)
    }
}