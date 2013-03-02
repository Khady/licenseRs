licenseRs
=========

licenseRs generates license files

    Usage: ./licenseRs [options]
    -h --help       Usage
    --proj          Project name, defaults to name of current directory
    --year          Copyright year
    --org           Organization, defaults to $USER
    --license       The license to generate, one of: agpl3, apache, bsd2, bsd3, cddl, cc0, epl, gpl2, gpl3, lgpl, mit, mpl
    --template      Path to license template file
    --vars          List template variables for specified license

This tool is inspired from [lice](http://github.com/jcarbaugh/lice)

Compiling
=========

    rustc licenseRs.rs

Requirements
============

 * Rust-0.6
