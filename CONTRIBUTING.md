<!-- omit in toc -->
# Contributing to Bizarre Friends

First off, thanks for taking the time to contribute! ❤️

Before you begin, please make sure you have read the [documentation](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/wiki).

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

To get started, please search for existing [Discussions](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/discussions) or [Issues](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/issues) that you can join.

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
>
> -   Star the project
> -   Refer this project in your project's readme
> -   Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->
## Table of Contents

- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features or Enhancements](#suggesting-features-or-enhancements)
  - [Improving The Documentation](#improving-the-documentation)
- [Modifying the code](#modifying-the-code)
- [Style Guides](#style-guides)
  - [Rust](#rust)
  - [Typescript](#typescript)
  - [Commits](#commits)
- [Join The Project Team](#join-the-project-team)
- [Attribution](#attribution)

## I Have a Question

-   Open a [Q&A](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/discussions/new?category=q-a).
-   Provide as much context as you can about what you're running into.
-   Provide the platform (Windows, Linux, MacOS, Android, iOS, Chrome, Firefox, etc.) and platform version you're using.

## I Want To Contribute

<!-- omit in toc -->
> ### Legal Notice 
>
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project [license](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/blob/main/LICENSE).

### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

-   Make sure that you are using the latest version.
-   To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friendsissues?q=label%3Abug).
-   Collect as much information as you can about the bug:
    -   Any error messages
    -   A stack trace (Traceback)
    -   Platform and platform version (Windows, Linux, MacOS, Android, iOS, Chrome, Firefox, etc.)
    -   Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
    -   Possibly your input and the output
    -   Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->
#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent to our [security](mailto:security+github@nibblesandmlemsexotics.com) email.

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

-   Open an [Bug Report](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/issues/new?assignees=&labels=bug&projects=Nibbles-and-Mlems-Exotics-LLC%2F2&template=bug_report.yaml&title=%5BBug%5D%3A+).
-   Explain the behavior you would expect and the actual behavior.
-   Please provide as much context as possible and describe the _reproduction steps_ that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
-   Provide the information you collected in the previous section.

Once it's filed:

-   The project team will label the issue accordingly.
-   A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
-   If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

### Suggesting Features or Enhancements

-   Suggest a change to existing functionality by opening an [Enhancement Request](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/issues/new?assignees=&labels=enhancement&projects=Nibbles-and-Mlems-Exotics-LLC%2F2&template=enhancement.yaml&title=%5BEnhancement%5D%3A+).
-   Suggest new functionality by submitting a [Feature Request](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/issues/new?assignees=&labels=feature&projects=Nibbles-and-Mlems-Exotics-LLC%2F2&template=feature_request.yaml&title=%5BFeature%5D%3A+)

<!-- omit in toc -->
#### How Do I Submit a Good Feature or Enhancement Request?

-   Ensure that your idea fits with the scope and aims of the project. It's up to you to make a strong case for the merits of this feature.
-   Use a clear and descriptive title to identify the suggestion.
-   Provide a detailed description of the suggested enhancement.
-   Explain the expected benefits of the change.
-   Identify who would benefit from the change (e.g. everyone, breeders, care providers).
-   You may want to include screenshots, wireframes, or diagrams which help you demonstrate the steps or point out the part which the suggestion is related to. Please do not include animated GIF files.

### Improving The Documentation

-   We use the GitHub wiki for our [documentation](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/wiki). To suggest changes please open an [Documentation Request](https://github.com/Nibbles-and-Mlems-Exotics-LLC/bizarre-friends/issues/new?assignees=&labels=documentation&projects=Nibbles-and-Mlems-Exotics-LLC%2F2&template=documentation.yaml&title=%5BDocumentation%5D%3A+).

## Modifying the code

-   Follow the guide for [Contributing to a project](https://docs.github.com/en/get-started/exploring-projects-on-github/contributing-to-a-project)

## Style Guides

### Rust

-   Use _rustfmt_
-   Follow the official [Rust Style Guide](https://doc.rust-lang.org/beta/style-guide/index.html) including:
    -   Four (4) spaces for indentation, no tabs
    -   Maximum line width of 100 characters
    -   Block indentation
    -   Include trailing commas
    -   Line comments (// ...) instead of block comments (/_ ... _/)
    -   Line doc comments (/// ...) instead of block doc comments (/\*_ ... _/)

### Typescript

-   Use _prettier_, a configuration file is included
-   Follow the official [Typescript style guide](https://ts.dev/style/).
-   For simplicity, four spaces (4) for indentation, no tabs
-   Include trailing commas
-   Maximum line width of 100 characters

### Commits

-   All commits MUST be linked to one or more Issues or Discussions.
-   All commit messages MUST use the following format:
    -   \<Subject\><br/>
        \<Description\><br/>
        \<Related Issues and Discussions\>
    -   The subject should be a one line description of the commit.
    -   The desciption should summarize the changes made.
    -   Related Issues and Discussions should be linked using GitHub # links.
-   Example:
    -   Fix Japanese translations<br/>
        Fix the Japanese translations for several animal names and traits to use the preferred Hiragana instead of Katakana.<br/>
        [#1]()

## Join The Project Team

-   Send an email to [Nibbles and Mlems Exotics LLC](mailto:join+github@nibblesandmlemsexotics.com)

## Attribution

This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
