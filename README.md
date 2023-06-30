# safe-package
safe-package is a security wrapper for your package manager. It keeps your build process safe from malicious and typo-squatted package dependencies, helping protect against code which exfiltrates secrets and sensitive data, code that takes control of your system, ransomware, and code that initiates malicious pull requests. 

## Installation


### From Crates.io
`cargo install safe-package
`

### From GitHub
Check [releases](https://github.com/arnica-io/SafePackage/releases) for the latest.

### From source
`git clone https://github.com/arnica-io/SafePackage`

`cd SafePackage`

`cargo build`

You'll need cargo and a handful of dependencies listed in the [Cargo.toml](https://github.com/arnica-io/SafePackage/blob/main/Cargo.toml) file.

## Running safe-package

Usage: safe-package [OPTIONS] [EXE_ARGS]...

###Arguments
  [EXE_ARGS]...  Arguments to the package manager

###Options:
*   -e, --exe <EXE>            The package manager to execute. If none is defined, the first ARG will be used
*   -r, --root-dir <ROOT_DIR>  The directory to chroot to
*   -k, --keep-env <KEEP_ENV>  A list of environment variables that the package manager needs
*   -u, --user <USER>          Who to run the package manager as
*   -h, --help                 Print help
*   -V, --version              Print version

## Configuration
safe-package looks in the following for configuration, in order:

1. /etc/safe-package/config.json
2. ~/.safe-package/config.json
3. $CWD/.safe-package/config.json
4. Command line arguments

Each location overlays on top of the previous. Lists of things like environment variables will stack. Singletons like the exe to execute and the user to run as will get replaced. That way for example if 90% of the code you write is node, you can just defene "exe": "/usr/bin/npm" in /etc/safe-package/config.json. Or you can skip all the config files and just define everything at the command line.

## Examples

The following runs npm with a root directory of /cellblock/npm as user 'nobody,' preserving the HTTPS_PROXY environment variable used by npm:

` safe-package -k HTTPS_PROXY -u nobody -r /cellblock/npm /usr/bin/npm install foobar
`

That's a lot to type out every time you want to try out a new package, so feel free to do make a configuation file in your home directory's .safe-package subdirectory:

` { "exe": "/usr/bin/npm", "user": "nobody", "root_dir": "/cellblock/npm" }`

And then put a handy alias in your shell's resource file:

`alias spm=safe-package`

And securely installing potentially malicious packages becomes as easy as what you're used to:

` spm install foobar`

## FAQ

### Q: When's that Windows version coming out?

I am an open source Unix developer. Get me in touch with an open source Windows developer and I'll see what we can hash out. 

### Q: Can't you use safe-package to wrap execution of things other than package managers?

Yes, and I don't have any problem with you finding nifty uses for it. I am in the business of building a world where software development happens unfettered by risk.

### Q: Why Rust and not Go?

I like Go pretty good and use it elsewhere. Go's coroutines and channels aren't useful for safe-package because I don't need to fan out any massive workloads. I need to clear an environment, isolate the file system, drop privs and execve. 

### Q: Why Rust and not...

It's going to be a pretty short application in any language. If I'm going to write a security tool that explicitly handles malicious package dependencies, I need to make sure that there's nothing I've done to make security worse. Rust is good about throwing potential defects in your face at build time, rather than at run time.

### Q: Why chroot and not ...

Chroot is a system call that was added to the Unix kernel in 1979. If your build system is a Unix that is newer than 1979, you have chroot already for free. Using something else would require installing something else. That said, there are a lot of nifty file system isolation tech out there, Thomas Ptacek recently wrote a nice blog post about some of them, and safe-package might include non-chroot options in the future.

### Q: Can you help me set up my chroot jail?

Yes! See the extras directory for some useful sample scripts. The key word is sample. Expect that you'll have to modify them to your liking.

### Q: So safe-package installs my package files in the chroot jail. How do I get them out?

You have options that depend on your tech stack. If all you're using is linux, you can replace your software installation directory with a symlink into the jail. If that seems messy to you, script up a synchronization step with something like `rsync -a` to get your files out of jail. Containery environments have containery solutions, like volumes. 

### Q: Why do I need root to run safe-package?

Root permissions are needed to call the chroot(2) system call and to drop privileges. If you don't want to do those things, you can still use safe-package to protect your environment variables:

`safe-package -k THIS -k THAT -k OTHER /usr/bin/pip3`

### Q: Can I run safe-package with sudo?

Yes.

### Q: I heard that chroot was insecure. Is it?

Chroot is only secure if you change your working directory and drop privileges after calling `chroot(2)`, which safe-package does. If you don't change the working directory, your package manager can still access files in the working directory. If you don't drop privileges to a non-root user, there will always be ways for root to break out of jail because root has the full power of the kernel at its disposal. 

Check [CVE Details](https://www.cvedetails.com/google-search-results.php?q=chroot&sa=Search). You'll see a kernel bug from 2016 that got Alan Cox hopping mad, a handful of implementation bugs in software like sudo that calls chroot, a few more implementation bugs in crusty old Unixes like SCO (ha!), and not much else.

### Q: Didn't Alan Cox say that chroot shouldn't be used for security? He seems authoritative.
Alan Cox said that chroot shouldn't be used for security purposes, and he is a net.god. Simson Garfinkel and Gene Spafford wrote about the use of chroot for security purposes in Practical Unix & Internet Security back in 1991. Carla Shroeder wrote about it in the first edition of The Linux Cookbook. It's been used extensively to great success in FTP applications, which, if you think about it, is kinda what a package manager is. 

But Shroeder, Spaf, and Garfinkel aren't net.gods like Alan Cox is. W. Richard Stevens wrote about using chroot for security purposes in Advanced Programming in the UNIX Environment, and Stevens is untouchable as far as I'm concerned. 

