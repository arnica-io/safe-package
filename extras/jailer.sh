#!/usr/bin/env bash

set -x

function handle_path_exe
{
    echo $PATH | tr ':' $'\n' | while read pathdir
    do
        if [ -x $pathdir/$1 ]
        then
            $0 "$jail" "$pathdir/$1"
        fi
    done
}

function handle_shebang()
{
    shebang=`head -1 $1`
    if ! echo "$shebang" | egrep '^#!'
    then
        echo "A script that doesn't start with a shebang. That's odd."
        return
    fi
    interpreter=`echo "$shebang" | cut -b2- | awk '{print $1}'`
    if echo $interpreter | grep "/usr/bin/env" 
    then
        handle_path_exe `echo "$shebang" | cut -b2- | awk '{print $2}'`
    fi
}

function exceptional_lib()
{
    if echo "$1" | grep libtinfo
    then
        $0 "$jail" /lib/terminfo
    fi
}

function exceptional_exe()
{
    true # No exceptional exes identified yet.
}

function exceptional_script()
{
    true # No exceptional scripts identified yet.
}

function exceptional_file()
{
    true # No exceptional files identified yet.
}

if [ $# -ne 2 ]
then
    echo "usage: $0 <src executable> <jail directory>"
    exit 2
fi

src=$1
jail=$2

# b is the basename, vim of /usr/bin/vim
b=`basename $src`

# d is the dirname, /usr/bin of /usr/bin/vim
d=`dirname $src`

if [ -e "$jail" && ! -d "$jail" ]
then
    echo "$jail already exists but is not a directory, exiting."

    exit 2
fi

if [ ! -e "$jail" ]
then
    echo "creating $jail"
    mkdir -p "$jail"
fi

#cd "$jail"

f=`file $src`
if echo "$f" | grep "symbolic link to"
then 
    symlink_src=`echo $f | awk '{print $NF}'`
    $0 "$jail" "$symlink_src"
    ln -s $jail/$symlink_src $jail/$src
elif echo "$f" | grep "script text executable"
then
    exceptional_script $b
    mkdir -p "$jail/$d"
    ln "$src" "$jail/$src"
    handle_shebang $src
elif echo "$f" | grep "executable"
then
    exceptional_exe $b
    echo jailing $src
    ln "$src" "$jail/$src"
    ldd $src | while read line
    do
        if echo "$line" | grep "linux.vdso.so.1"
        then
            continue # ignore, provided by the kernel
        elif echo "$line" | grep "=>"
        then
            $0 "$jail" `awk '{print $3}'` #
        else
            $0 "$jail" `awk '{print $1}'` #
        fi
    done

    if [ -d /usr/share/$b ]
    then
        echo Jailing /usr/share/$b
        $0 "$jail" "/usr/share/$b"
    fi
    if [ -d /etc/$b ]
    then
        echo Jailing /etc/$b
        $0 "$jail" "/etc/$b"
    elif echo "$f" | grep "shared object"
    then
        exceptional_lib $b
        mkdir -p "$jail/$d"
        ln $src $jail/$src
    else
        exceptional_file $b
        mkdir -p "$jail/$d"
        ln $src $jail/$src
    fi
fi
