mod article;
mod comment;
mod image;
mod user;

#[derive(Clone)]
pub struct Content {
    comment: comment::Comment,
    article: article::Article,
    image: image::Image,
    user: user::User,
}

impl Content {
    fn new(
        comment: comment::Comment,
        article: article::Article,
        image: image::Image,
        user: user::User,
    ) -> Self {
        Self {
            comment,
            article,
            image,
            user,
        }
    }

    fn comment(&self) -> comment::Comment {
        self.comment.clone()
    }

    fn article(&self) -> article::Article {
        self.article.clone()
    }

    fn image(&self) -> image::Image {
        self.image.clone()
    }

    fn user(&self) -> user::User {
        self.user.clone()
    }
}
