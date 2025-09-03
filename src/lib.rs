pub mod client;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::client::HydrusPermissions;

    use super::*;

    #[test]
    fn correct_permissions_number() {
        assert_eq!(HydrusPermissions::ImportAndEditURLs as u8, 0);
        assert_eq!(HydrusPermissions::ImportAndEditFiles as u8, 1);
        assert_eq!(HydrusPermissions::EditFileTags as u8, 2);
        assert_eq!(HydrusPermissions::SearchAndFetchFiles as u8, 3);
        assert_eq!(HydrusPermissions::ManagePages as u8, 4);
        assert_eq!(HydrusPermissions::ManageCookiesAndHeaders as u8, 5);
        assert_eq!(HydrusPermissions::ManageDatabase as u8, 6);
        assert_eq!(HydrusPermissions::EditFileNotes as u8, 7);
        assert_eq!(HydrusPermissions::EditFileRelationships as u8, 8);
        assert_eq!(HydrusPermissions::EditFileRatings as u8, 9);
        assert_eq!(HydrusPermissions::ManagePopups as u8, 10);
        assert_eq!(HydrusPermissions::EditFileTimes as u8, 11);
        assert_eq!(HydrusPermissions::CommitPending as u8, 12);
        assert_eq!(HydrusPermissions::SeeLocalPaths as u8, 13);
    }

    #[test]
    fn correct_url_encode() {
        let perms: [HydrusPermissions; 3] = [
            HydrusPermissions::ImportAndEditURLs,
            HydrusPermissions::ImportAndEditFiles,
            HydrusPermissions::SeeLocalPaths,
        ];
        let json_string = serde_json::to_string(&json!(perms)).unwrap();
        let encoded = urlencoding::encode(&json_string);

        assert_eq!(encoded, "%5B0%2C1%2C13%5D")
    }
}
