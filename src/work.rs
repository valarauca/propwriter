
use std::path::PathBuf;
use std::ffi::OsString;

use clap::ArgMatches;
use windows::Result;
use super::bindings::{
    Windows::Win32::Storage::StructuredStorage::PROPVARIANT,
    Windows::Win32::System::PropertiesSystem::{
        SHGetPropertyStoreFromParsingName,
        GPS_READWRITE,
        PROPERTYKEY,
        IPropertyStore,
        InitPropVariantFromStringAsVector,
        InitPropVariantFromUInt32Vector,
        PSGetPropertyKeyFromName,
        InitPropVariantFromPropVariantVectorElem,
    },
};

#[derive(Debug)]
pub struct WorkTodo {
    path: String,
    tags: Option<String>,
    artists: Option<String>,
    writers: Option<String>,
    producers: Option<String>,
    subtitle: Option<String>,
    title: Option<String>,
    series_name: Option<String>,
    year: Option<u32>,
    episode: Option<u32>,
    season: Option<u32>,
    genres: Option<String>,
}


fn flatten_multi_to_win_fmt<'a>(arg: Option<clap::Values<'a>>) -> Option<String> {

    fn remove_zero_length_strings<'b>(arg: &'b str) -> Option<&'b str> {
        let trimmed = arg.trim();
        if trimmed.len() == 0 {
            None
        } else {
            Some(trimmed)
        }
    }

    fn split_off<'a>(arg: &'a str) -> Vec<&'a str> {
        if arg.contains(':') {
            arg.split(':')
                .filter_map(remove_zero_length_strings)
                .collect::<Vec<&'a str>>()
        } else {
            vec![arg]
        }
    }

    match arg {
        Option::None => None,
        Option::Some(iterator) => {
            Some(iterator
                .filter_map(remove_zero_length_strings)
                .flat_map(split_off)
                .map(|x| x.to_lowercase())
                .collect::<Vec<String>>()
                .join(":"))
        }
    }
}

impl WorkTodo {

    pub fn new(cli_args: &ArgMatches) -> WorkTodo {
        WorkTodo{
            path: cli_args.value_of("file").unwrap().to_string(),
            tags: flatten_multi_to_win_fmt(cli_args.values_of("tags")),
            artists: flatten_multi_to_win_fmt(cli_args.values_of("artists")),
            producers: flatten_multi_to_win_fmt(cli_args.values_of("producers")),
            writers: flatten_multi_to_win_fmt(cli_args.values_of("writers")),
            genres: flatten_multi_to_win_fmt(cli_args.values_of("genre")),
            title: cli_args.value_of("title")
                .map(String::from),
            subtitle: cli_args.value_of("subtitle")
                .map(String::from),
            series_name: cli_args.value_of("series-name")
                .map(String::from),
            year: cli_args.value_of("year")
                .map(|x| u32::from_str_radix(&x, 10).ok())
                .flatten(),
            episode: cli_args.value_of("episode")
                .map(|x| u32::from_str_radix(&x,10).ok())
                .flatten(),
            season: cli_args.value_of("season")
                .map(|x| u32::from_str_radix(&x,10).ok())
                .flatten(),
        }
    }

    unsafe fn set_writers(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.writers {
            &Option::None => { }
            &Option::Some(ref x) => {
                set_string_vec_key(x, "writers", "System.Media.Writer", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_artists(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.artists {
            &Option::None => { }
            &Option::Some(ref artists) => {
                set_string_vec_key(artists, "artistts", "System.Author", store)?;
            }
        };
        Ok(())
    }


    unsafe fn set_tags(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.tags {
            &Option::None => { }
            &Option::Some(ref tags) => {
                set_string_vec_key(tags, "tags", "System.Keywords", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_series_name(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.series_name {
            &Option::None => { }
            &Option::Some(ref series_name) => {
                set_string_key(series_name, "series_name", "System.Media.SeriesName", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_title(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.title {
            &Option::None => { }
            &Option::Some(ref title) => {
                set_string_key(title, "title", "System.Title", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_subtitle(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.subtitle {
            &Option::None => { }
            &Option::Some(ref subtitle) => {
                set_string_key(subtitle, "subtitle", "System.Media.SubTitle", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_genres(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.genres {
            &Option::None => { }
            &Option::Some(ref genres) => {
                set_string_vec_key(genres, "genres", "System.Music.Genre", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_producers(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.producers {
            &Option::None => { }
            &Option::Some(ref producers) => {
                set_string_vec_key(producers, producers, "System.Media.Producer", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_year(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.year {
            &Option::None => { },
            &Option::Some(ref value) => {
                set_uint_key(value, "year", "System.Media.Year", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_season(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.season {
            &Option::None => { },
            &Option::Some(ref value) => {
                set_uint_key(value, "season", "System.Media.SeasonNumber", store)?;
            }
        };
        Ok(())
    }

    unsafe fn set_episode(&self, store: &IPropertyStore) -> std::result::Result<(),String> {
        match &self.episode {
            &Option::None => { },
            &Option::Some(ref value) => {
                set_uint_key(value, "episode", "System.Media.EpisodeNumber", store)?;
            }
        };
        Ok(())
    }

    pub fn do_work(&self) -> std::result::Result<(),String> {
        unsafe {
            let store: IPropertyStore = match SHGetPropertyStoreFromParsingName(self.path.as_str(), Option::None, GPS_READWRITE) {
                Ok(store) => store,
                Err(e) => return Err(format!("{:?} failed to open property store for {:?}", e, &self.path)),
            };
            self.set_producers(&store)?;
            self.set_title(&store)?;
            self.set_tags(&store)?;
            self.set_subtitle(&store)?;
            self.set_genres(&store)?;
            self.set_artists(&store)?;
            self.set_series_name(&store)?;
            self.set_writers(&store)?;
            self.set_episode(&store)?;
            self.set_year(&store)?;
            self.set_season(&store)?;
            match store.Commit() {
                Ok(()) => Ok(()),
                Err(e) => Err(format!("failed to commit error: {:?}", e))
            }
        }
    }
}

unsafe fn set_string_vec_key(string: &str, name: &str, key: &str, store: &IPropertyStore) -> std::result::Result<(),String> {
    let key = make_key(key)?;
    let value = match InitPropVariantFromStringAsVector(string) {
        Ok(value) => value,
        Err(e) => return Err(format!("{:?} failed to initialize {}'s vector, from string {:?}", e, name, string)),
    };
    set(store, &key, &value)?;
    Ok(())
}

unsafe fn set_string_key(string: &str, name: &str, key: &str, store: &IPropertyStore) -> std::result::Result<(),String> {
    let key = make_key(key)?;
    let value_1 = match InitPropVariantFromStringAsVector(string) {
        Ok(value_1) => value_1,
        Err(e) => return Err(format!("{:?} failed to init vector {:?}", e, string)),
    };
    let value_2 = match InitPropVariantFromPropVariantVectorElem(&value_1, 0) {
        Ok(value_2) => value_2,
        Err(e) => return Err(format!("{:?} failed to convert vec to element for {}", e, name)),
    };
    set(store, &key, &value_2)?;
    Ok(())
}


unsafe fn set_uint_key(numba: &u32, name: &str, key: &str, store: &IPropertyStore) -> std::result::Result<(),String> {
    let key = make_key(key)?;
    let value = match InitPropVariantFromUInt32Vector(numba, 1) {
        Ok(x) => x,
        Err(e) => return Err(format!("{:?} failed to initialize vector for {} from value: {:?}", e, name, numba))
    };
    let value_2 = match InitPropVariantFromPropVariantVectorElem(&value, 0) {
        Ok(value_2) => value_2,
        Err(e) => return Err(format!("{:?} failed to convert vec to element for {}", e, name)),
    };
    set(store, &key, &value_2)?;
    Ok(())
}

fn make_key(name: &str) -> std::result::Result<PROPERTYKEY,String> {
    unsafe {
        match PSGetPropertyKeyFromName(name) {
            Ok(x) => Ok(x),
            Err(e) => Err(format!("{:?} failed to get property key from name {:?}", e, name))
        }
    }
}

fn set(store: &IPropertyStore, key: &PROPERTYKEY, value: &PROPVARIANT)-> std::result::Result<(),String> {
    unsafe {
        match store.SetValue(key,value) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("{:?} failed to set {:?}", e, key))
        }
    }
}

