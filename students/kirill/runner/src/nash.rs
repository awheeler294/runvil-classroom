use reqwest::{Client, Method};
use runvil::{
    BoxFuture, Error, ErrorCode, ToolId, ToolInfo, ToolOutput, make_tool_error,
    packs::PackId,
    plugin::{Pack, PackInfo, PackRegistry, ToolContext},
};
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};

pub fn install(packs: &mut PackRegistry, server: &str) -> Result<(), Error> {
    let server = server.trim_end_matches('/').to_owned();
    let client = Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| make_tool_error(ErrorCode::ToolOther, error.to_string()))?;
    packs.insert(NASH, move |id: &PackId| {
        let parameters = json!({
            "type": "object",
            "properties": {
                "method": { "type": "string", "enum": ["GET", "PUT", "DELETE"] },
                "path": { "type": "string", "description": "Path on the Nash host, starting with /api/." },
                "body": { "type": "string", "description": "JSON text to send with a PUT." }
            },
            "required": ["method", "path"]
        });
        let tool = ToolInfo::new(
            ToolId::parse(&format!("{id}/request"))?,
            "Send one request to the Nash host and return its status and body as JSON.",
            parameters,
        );
        let pack = Nash {
            tool,
            client: client.clone(),
            server: server.clone(),
        };
        Ok(Arc::new(pack) as Arc<dyn Pack>)
    })
}

const NASH: PackInfo = PackInfo::new("kirill/nash").description("Send requests to the Nash host");

struct Nash {
    tool: ToolInfo,
    client: Client,
    server: String,
}

impl Pack for Nash {
    fn tools(&self) -> Vec<ToolInfo> {
        vec![self.tool.clone()]
    }

    fn call<'a>(
        &'a self,
        _context: ToolContext<'a>,
        args: Value,
    ) -> BoxFuture<'a, Result<ToolOutput, Error>> {
        Box::pin(async move {
            let invalid = |reason: &str| make_tool_error(ErrorCode::ToolInvalidArguments, reason);
            let method = match args["method"].as_str() {
                Some("GET") => Method::GET,
                Some("PUT") => Method::PUT,
                Some("DELETE") => Method::DELETE,
                _ => return Err(invalid("method must be GET, PUT or DELETE")),
            };
            let path = args["path"]
                .as_str()
                .filter(|path| path.starts_with("/api/"))
                .ok_or_else(|| invalid("path must start with /api/"))?;
            let mut request = self
                .client
                .request(method, format!("{}{path}", self.server));
            if let Some(body) = args["body"].as_str() {
                request = request
                    .header("content-type", "application/json")
                    .body(body.to_owned());
            }
            let unreachable = |error: reqwest::Error| {
                make_tool_error(
                    ErrorCode::ToolTransport,
                    format!("the Nash host at {} did not answer: {error}", self.server),
                )
            };
            let response = request.send().await.map_err(unreachable)?;
            let status = response.status().as_u16();
            let body = response.text().await.map_err(unreachable)?;
            Ok(ToolOutput::trusted(
                json!({ "status": status, "body": body }).to_string(),
            ))
        })
    }
}
