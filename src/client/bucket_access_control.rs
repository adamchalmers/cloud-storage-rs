use crate::{models::{create, BucketAccessControl, ListResponse, Entity, Response}, Error};

/// Operations on [`BucketAccessControl`](BucketAccessControl)s.
#[derive(Debug)]
pub struct BucketAccessControlClient<'a> {
    pub(crate) client: &'a super::client::Client,
    pub(crate) bucket_acl_url: String
}

impl<'a> BucketAccessControlClient<'a> {
    /// Create a new `BucketAccessControl` using the provided `create::BucketAccessControl`.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, create::BucketAccessControl};
    /// use cloud_storage::bucket_access_control::{Role, Entity};
    ///
    /// let client = Client::default();
    /// let new_bucket_access_control = create::BucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// client.bucket_access_control("my_bucket").create_using(&new_bucket_access_control).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_using(
        &self,
        new_bucket_access_control: &create::BucketAccessControl,
    ) -> Result<BucketAccessControl, Error> {
        let headers = self.client.get_headers().await?;
        let result: crate::models::Response<BucketAccessControl> = self.client.reqwest.post(&self.bucket_acl_url).headers(headers).json(new_bucket_access_control).send().await?.json().await?;
        Ok(result?)
    }

    /// Returns all `BucketAccessControl`s related to this bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::BucketAccessControl;
    ///
    /// let client = Client::default();
    /// let acls = client.bucket_access_control("my_bucket").list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<BucketAccessControl>, Error> {
        let headers = self.client.get_headers().await?;
        let response = self.client.reqwest.get(&self.bucket_acl_url).headers(headers).send().await?;

        let object = response.json::<Response<ListResponse<BucketAccessControl>>>().await??.items;
        Ok(object)
    }

    /// Returns the ACL entry for the specified entity.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let controls = client.bucket_access_control("my_bucket").read(&Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(&self, entity: &Entity) -> Result<BucketAccessControl, Error> {
        let url = format!(
            "{}/{}",
            self.bucket_acl_url,
            crate::percent_encode(&entity.to_string())
        );
        let headers = self.client.get_headers().await?;
        let result: crate::models::Response<BucketAccessControl> = self.client.reqwest.get(&url).headers(headers).send().await?.json().await?;
        Ok(result?)
    }

    /// Update this `BucketAccessControl`.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let mut acl = client.bucket_access_control("my_bucket").read(&Entity::AllUsers).await?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// client.bucket_access_control().update(&acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        bucket_access_control: &BucketAccessControl,
    ) -> Result<BucketAccessControl, Error> {
        let url = format!(
            "{}/{}",
            self.bucket_acl_url,
            crate::percent_encode(&bucket_access_control.entity.to_string()),
        );
        let headers = self.client.get_headers().await?;
        let result: crate::models::Response<BucketAccessControl> = self.client.reqwest.put(&url).headers(headers).json(bucket_access_control).send().await?.json().await?;
        Ok(result?)
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let my_bucket = client.bucket_access_control("my_bucket");
    /// let controls = my_bucket.read(&Entity::AllUsers).await?;
    /// my_bucket.delete(controls).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, bucket_access_control: BucketAccessControl) -> Result<(), Error> {
        let url = format!(
            "{}/{}",
            self.bucket_acl_url,
            crate::percent_encode(&bucket_access_control.entity.to_string()),
        );
        let headers = self.client.get_headers().await?;
        let response = self
            .client
            .reqwest
            .delete(&url)
            .headers(headers)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}
