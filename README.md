# rust-jira-cli
rust cli for creating epics, stories and updating their status using rust, some insight points -

1. difference between ok_or and ok_or_else - ok_or taken an error object while ok_or_else taken closure which generated error object only in case of error path
2. "anyhow" usage saves some boiler plate code - using Result object from anyhow library and using anyhow macro saves some code for error handling and makes it easy
3. if you see I have refactored tests and test utils in a separate file as compared to the actual code files
4. usage of MockDB object for mocking db calls implements same interface and makes testing possible
5. if you see pages is an array of Page trait object in pub struct Navigator because the array can push into it any concrete object which implenentes Page interface
6. "?" operator is used for coercing and propogation of error in neat way
7. fmt::Display trait is being implelmented for the Status struct which will be used when .toString method is called over it

