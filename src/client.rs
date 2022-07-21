use crate::models::Checklist;

pub struct Client {
    auth: String,
}

pub struct GetChecklistsOptions {
    archived: bool,
    order: Option<String>,
    skip_stats: bool,
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
        let checklists: Vec<Checklist> = self.get_checklists_with_options(GetChecklistsOptions {
            archived: false,
            order: None,
            skip_stats: false,
        })?;

        Ok(checklists)
    }

    pub fn get_checklists_with_options(&self, options: GetChecklistsOptions) -> Result<Vec<Checklist>, ureq::Error> {
        let mut request = ureq::get("https://checkvist.com/checklists.json")
            .set("Authorization", &self.auth);

        if options.archived {
            request = request.query("archived", "true");
        }

        if let Some(order) = options.order {
            request = request.query("order", &order);
        }

        if options.skip_stats {
            request = request.query("skip_stats", "true");
        }

        let checklists: Vec<Checklist> = request.call()?.into_json()?;

        Ok(checklists)
    }

    pub fn get_checklist(&self, id: u32) -> Result<Checklist, ureq::Error> {
        let url = format!("https://checkvist.com/checklists/{}.json", id);
        let checklist: Checklist = ureq::get(&url)
            .set("Authorization", &self.auth)
            .call()?
            .into_json()?;
        Ok(checklist)
    }
}