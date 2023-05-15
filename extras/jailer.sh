#!/usr/bin/env bash

#set -x

function handle_path_exe
{
    echo $PATH | tr ':' $'\n' | while read pathdir
    do
        if [ -x $pathdir/$1 ]
        then
            $0 "$pathdir/$1" "$jail" 
        fi
    done
}

function handle_shebang()
{
    shebang=`head -1 $1`
    if ! echo "$shebang" | egrep -q '^#!'
    then
        echo "A script that doesn't start with a shebang. That's odd."
        return
    fi
    interpreter=`echo "$shebang" | cut -b2- | awk '{print $1}'`
    if echo $interpreter | grep -q "/usr/bin/env" 
    then
        handle_path_exe `echo "$shebang" | cut -b2- | awk '{print $2}'`
    fi
}

function exceptional_lib()
{
    if echo "$1" | grep -q libtinfo
    then
        $0 /lib/terminfo "$jail" 
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

if [ -f "$jail" ]
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

# Symbolic link to a absolute path
if echo "$f" | grep -q "symbolic link to /"
then 
    symlink_src=`echo $f | awk '{print $NF}'`
    mkdir -p "$jail/$d"
    $0 "$symlink_src" "$jail" 
    echo "Jailing $src"
    ln -s ${symlink_src} ${jail}${src}
# Symbolic link to a relative path
elif echo "$f" | grep -q "symbolic link to"
then 
    symlink_src=$d/`echo $f | awk '{print $NF}'`
    mkdir -p "$jail/$d"
    $0 "$symlink_src" "$jail" 
    cd ${jail}/$d
    echo "Jailing $src"
    ln -s ${symlink_src} ${b}
    cd -
elif echo "$f" | egrep -q "/usr/bin/env [a-zA-Z0-9_-]+ script"
then
    exceptional_script $b
    if [ ! -f "$jail/$src" ]
    echo "Jailing $src"
    then
        mkdir -p "$jail/$d"
        ln "$src" "$jail/$src"
    fi
    handle_shebang $src
elif echo "$f" | grep -q "text executable"
then
    exceptional_script $b
    echo "Jailing $src"
    if [ ! -f "$jail/$src" ]
    then
        mkdir -p "$jail/$d"
        ln "$src" "$jail/$src"
    fi
    handle_shebang $src
elif echo "$f" | grep -q "executable"
then
    exceptional_exe $b
    echo Jailing $src
    if [ ! -f "$jail/$src" ]
    then
        mkdir -p "$jail/$d"
        ln "$src" "$jail/$src"
    fi
    ldd $src | while read line
    do
        if echo "$line" | grep -q "linux.vdso.so.1"
        then
            continue # ignore, provided by the kernel
        elif echo "$line" | grep -q "=>"
        then
            $0 `echo $line | awk '{print $3}'` "$jail" #
        else
            $0 `echo $line | awk '{print $1}'` "$jail" #
        fi
    done

elif echo "$f" | grep -q "shared object"
then
    exceptional_lib $b
    echo "Jailing $src"
    if [ ! -f "$jail/$src" ]
    then
        mkdir -p "$jail/$d"
        ln "$src" "$jail/$src"
    fi
elif echo "$f" | grep -q "directory"
then
    if [ ! -d $jail/$src ]
    then
        echo Jailing $src
        mkdir -p $jail/$src
        find $src | while read file
        do
            $0 $file $jail
        done
    fi
elif echo "$f" | grep -q "character special"
then
    echo "$f" | awk '{print $NF}' | 
else
    exceptional_file $b
    echo Jailing $src
    if [ ! -f "$jail/$src" ]
    then
        mkdir -p "$jail/$d"
        ln "$src" "$jail/$src"
    fi
fi 

if [ -f "$src" ]
then
    if [ -d /usr/share/$b ]
    then
        echo Jailing /usr/share/$b
        $0 "/usr/share/$b" "$jail" 
    fi

    if [ -d /etc/$b ]
    then
        echo Jailing /etc/$b
        $0 "/etc/$b" "$jail" 
    fi
fi
