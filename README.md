# hygge

---

    K=hash(V)
    V=V

So uh.. yeah. HashSetMap. ⛓️

---

A hashmap where the hash is precomputed and used as the key.

I don't actually override the hash _in_ the hashmap in this implementation.


If hasher allowed more than u64 output, this would be easier.

At least one method with mroe than -> u64.

Instead, the generic is limited to only that.

Being able to fit 160-512 bits would be make this nice.

As is, the generic version uses only left u64 bits,

and the specific version I need uses SHA-1, 160 bits.

(SHA-1 is only used because of git using it.)

git now supports sha256, but github does not.



*** Author is new to rust, this is a side project,
any advice on improving the ergonomics or practices
would be appreciated ***

---

### Making Changes

1. Fork the repository.

2. Create a new feature branch.

3. Make your changes. Ensure that there are no build errors by running the project with your changes locally.

4. Open a pull request with a name and description of what you did. You can read more about working with pull requests on GitHub [here](https://help.github.com/en/articles/creating-a-pull-request-from-a-fork).

5. A maintainer will review your pull request and may ask you to make changes.

# IMPORTANT NOTE:

Please note that this project is released with a
Contributor Code of Conduct, located in the root
directory of this project under the file name:

 - `CODE_OF_CONDUCT.md`

By participating in this project you agree to abide by its terms.

Thank you for your participation.

### Open Issues

If you're ready to contribute, start by looking at our open issues tagged as [`help wanted`](../../issues?q=is%3Aopen+is%3Aissue+label%3A"help+wanted") or [`good first issue`](../../issues?q=is%3Aopen+is%3Aissue+label%3A"good+first+issue").

You can comment on the issue to let others know you're interested in working on it or to ask questions.

### Bugs / Issues

When reporting a bug or unexpected behaviour in a project, make sure your issue describes steps to reproduce the behaviour, including the platform you were using, what steps you took, and any error messages.

Reproducible bugs will be tagged as `bug` and their status will be updated in the comments of the issue.

### Wontfix

Issues will be closed and tagged as `wontfix` if we decide that we do not wish to implement it, usually due to being misaligned with the project vision or out of scope. We will comment on the issue with more detailed reasoning.

### Feature Requests

If you have ideas or how to improve our projects, you can suggest features by opening a GitHub issue. Make sure to include details about the feature or change, and describe any uses cases it would enable.

Feature requests will be tagged as `enhancement` and their status will be updated in the comments of the issue.

## Licensing

Unless otherwise specified, all DEVELOPING.TODAY LLC open source projects shall comply with the Rust standard licensing model (MIT + Apache 2.0) and are thereby licensed under a dual license, allowing licensees to choose either MIT OR Apache-2.0 at their option.

## Contributor Terms

Thank you for your interest in this DEVELOPING.TODAY LLC open source project. By providing a contribution (new or modified code, other input, feedback or suggestions etc.) you agree to these Contributor Terms.

You confirm that each of your contributions has been created by you and that you are the copyright owner. You also confirm that you have the right to provide the contribution to us and that you do it under the Rust dual licence model (MIT + Apache 2.0).

If you want to contribute something that is not your original creation, you may submit it to DEVELOPING.TODAY LLC separately from any contribution, including details of its source and of any license or other restriction (such as related patents, trademarks,  agreements etc.)

Please also note that our projects are released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md) to ensure that they are welcoming places for everyone to contribute. By participating in any DEVELOPING.TODAY LLC open source project, you agree to keep to the Contributor Code of Conduct.
