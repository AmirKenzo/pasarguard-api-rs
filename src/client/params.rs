use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleListParams {
    pub ids: Option<Vec<i64>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub all: Option<bool>,
}

impl Default for SimpleListParams {
    fn default() -> Self {
        Self {
            ids: None,
            offset: None,
            limit: None,
            search: None,
            sort: None,
            all: Some(false),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageParams {
    pub period: Option<Period>,
    pub node_id: Option<i64>,
    pub group_by_node: Option<bool>,
    pub start: Option<PanelDateTime>,
    pub end: Option<PanelDateTime>,
}

impl Default for UsageParams {
    fn default() -> Self {
        Self {
            period: Some(Period::Hour),
            node_id: None,
            group_by_node: Some(false),
            start: None,
            end: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubUsageParams {
    pub period: Option<Period>,
    pub start: Option<PanelDateTime>,
    pub end: Option<PanelDateTime>,
}

impl Default for SubUsageParams {
    fn default() -> Self {
        Self {
            period: Some(Period::Hour),
            start: None,
            end: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
        }
    }
}

pub(crate) fn usage_query(params: &UsageParams) -> Query {
    Query::new()
        .opt("period", params.period)
        .opt("node_id", params.node_id)
        .opt("group_by_node", params.group_by_node)
        .opt("start", params.start.as_ref())
        .opt("end", params.end.as_ref())
}

pub(crate) fn simple_list_query(params: &SimpleListParams) -> Query {
    Query::new()
        .list("ids", params.ids.as_deref())
        .opt("offset", params.offset)
        .opt("limit", params.limit)
        .opt("search", params.search.as_deref())
        .opt("sort", params.sort.as_deref())
        .opt("all", params.all)
}
