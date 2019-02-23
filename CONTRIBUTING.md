# Contributing

Thank you for considering contributing to this project!


When contributing to this repository, please first discuss the change you wish to 
make via Matrix, issue, email, or any other method with the owners of this repository
before making a change. 

Our Matrix room is at #rust-native-ui:matrix.nora.codes.

Please note we have a code of conduct, please follow it in all your interactions 
with the project.

## Submitting issues

When you submit an issue, please make sure it is **clear** and **concise**.
Including code examples is much appreciated for bug reports, but if at all possible
please remove extraneous details from your example code.

Feature requests are welcome but be aware that due to time constraints we may not
be able to implement all features in a timely fashion. Also, be aware that `iui`
is constrained by the underlying `ui` library, so even if we are interested in
implementing a feature, it is possible we will have to wait a long time for
a corresponding implemention in `ui`.

## Submitting pull requests

Thank you for submitting code! We will work to get your changes merged as fast as
possible. To facilitate this, consider the following steps:

* Run `rustfmt` or `cargo fmt` on your code. This will prevent style nitpicks
from slowing down your PR.
* Mark unsafe functions as `unsafe`. This includes _anything_ with potential
undefined behavior, not just memory unsafety.
* Document your code! This is the number one holdup for pull requests, especially
large ones. Don't forget to insert appropriate entries into the changelog.
* If implementing a new feature from `ui`, please mention the stability of that
feature in your pull request. We are fine with implementing unstable APIs from
`ui`, but it's important to mark such APIs as unstable.

## Changelog and Versioning

This project adheres to [SemVer](https://semver.org) and uses a 
[KeepAChangelog](https://keepachangelog.com)-style changelog at 
[CHANGELOG.md](CHANGELOG.md). All PRs MUST include editing the changelog with any
new API surface, deprecations, fixes, et cetera.

## Code of Conduct

### Our Pledge

In the interest of fostering an open and welcoming environment, we as
contributors and maintainers pledge to making participation in our project and
our community a harassment-free experience for everyone, regardless of age, body
size, disability, ethnicity, gender identity and expression, level of experience,
nationality, personal appearance, race, religion, or sexual identity and
orientation.

### Our Standards

Examples of behavior that contributes to creating a positive environment
include:

* Using welcoming and inclusive language
* Being respectful of differing viewpoints and experiences
* Gracefully accepting constructive criticism
* Focusing on what is best for the community
* Showing empathy towards other community members

Examples of unacceptable behavior by participants include:

* The use of sexualized language or imagery and unwelcome sexual attention or
advances
* Trolling, insulting/derogatory comments, and personal or political attacks
* Public or private harassment
* Publishing others' private information, such as a physical or electronic
  address, without explicit permission
* Other conduct which could reasonably be considered inappropriate in a
  professional setting

### Our Responsibilities

Project maintainers are responsible for clarifying the standards of acceptable
behavior and are expected to take appropriate and fair corrective action in
response to any instances of unacceptable behavior.

Project maintainers have the right and responsibility to remove, edit, or
reject comments, commits, code, wiki edits, issues, and other contributions
that are not aligned to this Code of Conduct, or to ban temporarily or
permanently any contributor for other behaviors that they deem inappropriate,
threatening, offensive, or harmful.

### Scope

This Code of Conduct applies both within project spaces and in public spaces
when an individual is representing the project or its community. Examples of
representing a project or community include using an official project e-mail
address, posting via an official social media account, or acting as an appointed
representative at an online or offline event. Representation of a project may be
further defined and clarified by project maintainers.

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported by contacting the project team on GitHub or via [email](mailto:lfstindall@gmail.com). All
complaints will be reviewed and investigated and will result in a response that
is deemed necessary and appropriate to the circumstances. The project team is
obligated to maintain confidentiality with regard to the reporter of an incident.
Further details of specific enforcement policies may be posted separately.

Project maintainers who do not follow or enforce the Code of Conduct in good
faith may face temporary or permanent repercussions as determined by other
members of the project's leadership.

### Attribution

This Code of Conduct is adapted from the [Contributor Covenant][homepage], version 1.4,
available at [http://contributor-covenant.org/version/1/4][version]

[homepage]: http://contributor-covenant.org
[version]: http://contributor-covenant.org/version/1/4/
