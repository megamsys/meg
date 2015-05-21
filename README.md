meg installs your apps and services in [Megam](https://www.megam.io).

Learn more at http://doc.megam.io/.

## Installing meg in linux


```sh
\curl -sSL https://get.megam.io.io | bash -s stable
```

## Compiling meg

meg requires the following tools and packages to build:

* `rustc`

meg can then be compiled like many other standard unix-like projects:

```sh
git clone https://github.com/megamsys/meg
cd meg
cargo build
```

## Commands

meg is designed to be extensible with new subcommands without having to modify meg itself. See [the devcenter page](https://devcenter.megam.io) for more details and a list of known community-developed subcommands.

We are glad to help if you have questions, or request for new features..

[twitter @megamsys](http://twitter.com/megamsys) [email support@megam.io](<support@megam.io>)




# License

|                      |                                          |
|:---------------------|:-----------------------------------------|
| **Author:**          | Kishorekumar Neelamegam (<nkishore@megam.io>)
|                      | Raj Thilak (<rajthilak@megam.io>)
|                      | Yeshwanth Kumar (<getyesh@megam.io>)
| **Copyright:**       | Copyright (c) 2013-2015 Megam Systems.
| **License:**         | Apache License, Version 2.0

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
