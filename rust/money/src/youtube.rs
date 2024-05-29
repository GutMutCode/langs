#![allow(unused)]

struct Principle {
    similarity: u32,
    past_date: u32,
    previous_viewed: bool,
    previous_like: bool,
    subscribed: bool,
}

struct Efficacy {
    view: u32,
    like: u32,
    subscriber: u32,
    similarity: u32,
    past_date: u32,
}

struct Video {
    upload_date: String,
    title: String,
    description: String,
    url: String,
    thumbnail: String,
    channel: Channel,
    views: u32,
    likes: u32,
}

struct Channel {
    name: String,
    subscribers: u32,
    videos: Vec<Video>,
}

impl Channel {
    fn strategy(&self) -> Efficacy {
        if self.subscribers > 1000 {
            Efficacy {
                view: 100,
                like: 100,
                subscriber: 100,
                similarity: 100,
                past_date: 100,
            }
        } else {
            Efficacy {
                view: 100,
                like: 0,
                subscriber: 0,
                similarity: 100,
                past_date: 100,
            }
        }
    }
}

fn home_expose() {
    let channel = Channel {
        name: String::from("Skoler"),
        subscribers: 100,
        videos: Vec::new(),
    };

    let target_vedio = Video {
        upload_date: String::from("2020-01-01"),
        title: String::from("Skoler"),
        description: String::from("Skoler"),
        url: String::from("Skoler"),
        thumbnail: String::from("Skoler"),
        channel,
        views: 100,
        likes: 100,
    };
}
