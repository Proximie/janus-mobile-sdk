# Janus Mobile SDK Change/Release Process
The following steps give an outline of the process to create a new tagged release for the Janus Mobile SDK. In particular, the most likely scenario is when an update to the internal jarust dependency has been released upstream and those changes are needed here.

1. Make changes to janus-mobile-sdk. This may include updating the jarust dependency version which can be done in `/rslib/Cargo.toml`
2. After changes completed run locally:

Create native Apple platforms bindings. Disregard any keychain alerts that may pop-up, they are not needed
`just apple`

Create native Android platform bindings.
`just android`

3. Check all these changes in on a new branch and raise a PR.
4. After PR is merged we can locally run:

`just apple -r`

This will package up a release .xcframework, tag the new version for Swift Package Manager (SPM), and create a github release with these artifacts on the repo.

Details for these recipes can be found under `.justfile`.

**Note:** You need valid repo permissions to run `just apple -r` and this part of the process is planned to be evolved into a CI action.