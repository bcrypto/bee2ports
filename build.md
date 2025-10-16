# Swift

## Install

Swift don't need a wrapper for C code from version 4.2. 
Umbrella header and module.modulemap files should be in a separate directory.

## Build
```
cd bee2swift
swift build 
```
## Test
```
swift run 
```


# C#

## Install

1. Compile SWIG interface:
```
swig -csharp -o bee2net/bee2_wrap.c -outdir bee2net -outfile bee2cs.cs -dllimport bee2wrap bee2.i
```
2. Compile and build wrapper library:
```
mkdir -p bee2net/runtimes/linux-x64/native/
gcc bee2net/bee2_wrap.c --shared -lbee2_static -o bee2net/runtimes/linux-x64/native/bee2wrap.so
```
3. Compile C# project:
```
cd bee2net
dotnet build
```

## Test
```
cd test/test_csharp
dotnet build
dotnet run
```