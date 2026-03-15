use {
    crate::{
        tmdb::{
            client::TmdbClient,
            model::{Paginated, SearchMovie, SearchTv},
            utils::ApiResult,
        },
        tui::{Context, popup::Popup},
    },
    std::{borrow::Cow, fmt::Write},
};

pub trait SearchType: Sized {
    fn display(&self) -> Cow<'_, str>;
    fn details(&self) -> Option<Cow<'_, str>>;
    fn details_popup(&self, context: Context) -> Popup;
    fn search(client: &mut TmdbClient, query: String, page: i32) -> usize;
    fn results(client: &mut TmdbClient, id: usize) -> Option<ApiResult<Paginated<Self>>>;
}

impl SearchType for SearchMovie {
    fn display(&self) -> Cow<'_, str> {
        let mut result = self.title.clone();
        if let Some(d) = self.release_date {
            write!(result, " ({d})").unwrap();
        }
        if self.title != self.original_title {
            write!(result, " ({}: {})", self.original_language, self.original_title).unwrap();
        }
        Cow::Owned(result)
    }

    fn details(&self) -> Option<Cow<'_, str>> {
        match self.overview.as_str() {
            "" => None,
            s => Some(Cow::Borrowed(s)),
        }
    }

    fn details_popup(&self, context: Context) -> Popup { Popup::confirm_movie(self.id, context) }

    fn search(client: &mut TmdbClient, query: String, page: i32) -> usize {
        client.search_movie(query, page)
    }

    fn results(client: &mut TmdbClient, id: usize) -> Option<ApiResult<Paginated<Self>>> {
        client.search_movie_results(id)
    }
}

impl SearchType for SearchTv {
    fn display(&self) -> Cow<'_, str> {
        let mut result = self.name.clone();
        if let Some(d) = self.first_air_date {
            write!(result, " ({d})").unwrap();
        }
        if self.name != self.original_name {
            write!(result, " ({}: {})", self.original_language, self.original_name).unwrap();
        }
        Cow::Owned(result)
    }

    fn details(&self) -> Option<Cow<'_, str>> {
        match self.overview.as_str() {
            "" => None,
            s => Some(Cow::Borrowed(s)),
        }
    }

    fn details_popup(&self, context: Context) -> Popup { Popup::confirm_series(self.id, context) }

    fn search(client: &mut TmdbClient, query: String, page: i32) -> usize {
        client.search_tv(query, page)
    }

    fn results(client: &mut TmdbClient, id: usize) -> Option<ApiResult<Paginated<Self>>> {
        client.search_tv_results(id)
    }
}
