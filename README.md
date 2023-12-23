# VoxelOptimizer
This program was made to optimize the meshes that are exported by **MagicaVoxel** (software made by Ephtracy), written in rust, it is the **fastest** and, thanks to some clever optimizations and tricks, it is also the **best** at compressing even if it is a **lossless compression**, which means that the quality of the mesh isn't traded with the speed of the execution of the program. If you tried to export a mesh using magicavoxel you would know that the internal mesh exporter is pretty inefficient and it is not ideal for gamedev. For this reason and because I was unable to find a program such as this with the characteristics I had in mind this project was born. It is pretty similiar to an addon made for blender (Vox Cleaner V2 by Farhan) but it differs for many reasons:
1. Completely free
2. You don't need blender
3. Low ram usage
4. Low disk usage
5. Easy to use
6. multithreaded so that converting many models is blazingly fast

# Compatibility
Before explaining how it works I wanted to say that this program unfortunately only works for **windows**, if you know a little bit of rust you can contribute to other major platforms such as linux and mac. Also while this program doesn't use much cpu the better the cpu the faster will be the processes, the cpu also has to support multithreading to a certain capacity (Most of the cpu's will do the job). Last but not least to run this program you need a minimal amount of ram but it has to be at least as big as the models you are compressing (for example: you are converting 10 models 50mb each, 1 free gb of ram is reccomended).

# Usage and benchmarks
To get started just download the latest release (release v1.0) and extract it in a folder or on your desktop, the important thing is that both the folder "src" and voxeloptimizer.exe are on the same directory (whether it is on the desktop or in another folder). To run the program double click onto the executable and two windows will open, ignore the black one.
You'll have something like this:![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/9c930e08-efdb-42a0-88d3-89a8794076ba).
As you can see on the top there are instructions to follow, however there are many options which I will explain later below with also a series of benchmarks (tests to check how quick and efficient are the various settings). 

To convert models to an optimized and superior form you first have to create the models in magicavoxel and export them using the second option (.ply) and then drag and drop every file you want to convert (you can and you should do more than one at the time, tip: Control + A selects all the files in a folder) like so:

https://github.com/davidevofficial/voxel_optimizer/assets/127616649/4568ff63-293d-4748-83a3-ced18711c548

The default options are the best if you care about output file size, however depending on your needs you might need to change some, so here is every setting and its explanation pros and cons.

## Cross-overlapping optimization

This setting changes the way the algorithm works while reducing the amount of vertices, to explain how it works here are some examples:

Let's say you have a cross:![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/6ce2e925-1da8-4e6e-8920-63c1ae0c1d8a) 


Without the option it would be divided like so: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/1b3778b5-1a3c-4683-b68a-49827089e208) (3 cubes)


With the option the green and blue part become united: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/1c030205-fb38-41de-a3a8-8a0dc7afcd33) (2 overlapping cubes).

Reccomended: ON

### Pros: 
1. Greatly reduces File Size
### Cons: 
1. Slightly slower
2. The cubes overlap (in some software this results in bad behaviour)


## Enable de-cull optimization

The ply magicavoxel optimizes meshes when exporting such that a cube full inside is actually a cube empty inside but since you can't see it it doesn't matter except that it does if you have to compress it with this program.

Reccomended: ON

### Pros: 
1. Greatly reduces File Size
### Cons: 
1. Slightly slower

## Enable solid colours to be one pixel on the texture map

Behaviour when off: 

This (8x8 square):

![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/47b9fa5e-6b8a-468d-a90d-1c267eb24506) 

Becomes this on the texture map (8x8 square): 

![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/0dd6b5e0-2af5-4b50-8846-3f13c814ee21)


Behaviour when on: 

This (8x8 square):

![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/47b9fa5e-6b8a-468d-a90d-1c267eb24506) 

Becomes this on the texture map (1x1 square):

![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/85cc83ad-842a-497c-ac44-f1d2f1c4066a)

Reccomended: ON

### Pros: 
1. Greatly reduces File Size
2. Can use the next setting (pattern matching) at its fullest
### Cons: 
1. Slightly slower
2. Cannot manually modify the texture of the face since if you modify a pixel you modify all the face

## Pattern matching

Depending on the level of pattern matching each texture will be flipped, rotated and then compared to each other if two are equal than both faces will share the same region on the texture map:

![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/b179bbf3-3904-4052-808f-871a664f2994)

This image shows all 4 levels of pattern matching

Reccomended: 3 if you don't have to manually edit textures.

### Pros: 
1. Greatly reduces File Size
### Cons: 
1. Anything higher than 0 makes it O(n^2) slower where n is the number of textures, therefore it makes it greatly slower
2. Cannot manually modify the texture of a face without modifying the texture of all the faces equal to that one.

## Enable manual settings of the precision levels

with precisions level I mean the amount of digits after the dot in the output .obj file for each vt, in this image that number is 3: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/96b4f7d4-b264-480e-8f6a-3b5457c23ab6)

If the setting is off the program automatically detects the amount of digits otherwise you can specify it yourself

Reccomended: OFF

### Pros if it is on: 
1. Manually set digits numbers
2. more control
### Pro if it is off: 
1. You don't have to manyally set digits numbers

## Background Colour


select the colour of the pixels not used but present in the texture map.


How to use it: If you have a small palette it can save a really small amount of disk space if you use as a background the same colour as one present in the palette.

Reccomended: The same as the most used colour in the model

## Y vector is up

If you select it the programs that use the Y vector as the up vector will successfully open the model in the right orentation.

Reccomended: Highly dependent on the program you want to use the output in

## Origin is the center of the model

If you select it the model vertices will not have their position based on their magicavoxel position.

Off: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/cfd05e92-1fe6-4a07-9ed0-90f1440caaee)

On: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/78acbc1f-142d-44a0-b99a-94137e6b1325)

### Pros: 
1. Saves a little bit amount of disk space (especially if there are many small models created all over the place in magicavoxel)
### Cons: 
1. 2 models will not have their positions relative one to the other when importing the model in other programs 

## Enable UV debug mode

If you enable this all models will have this texture ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/08db6b00-7758-45e3-a3a6-7b16e51b10d5)
 correctly applied to each face:

## Convert

After the settings you should choose a directory where the output will be written to and then click the convert button. Once you are finished (The program should have a message that notifies you of that, if it doesn't move the mouse) you can kill the program, just close the Black window (command prompt).

## Benchmarks

To benchmark I'll use the .vox files that magicavoxel comes by default and I'll compare the .obj of magicavoxel with the .obj of my program 
![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/af857bf0-165c-4f81-b837-2489624a2516)


Here are the results: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/399a6c50-14c5-401f-bc97-3b4995e0a816)

As you can see the VoxelOptimizer output is clearly smaller than magicavoxel's one, it took my program about 45 seconds which is a little more than magicavoxel. In the benchmark folder of this repository you can find all of the data (.vox files, .ply files, my output, magica output).


# License and contributions
I would be glad for any pull request, discussion, issues you have to make this program better and also a little help for porting this to mac and linux (or maybe a wasm version in the future). 

License: you may modify and copy for private use the software but you cannot redistribute or sell it.

If you have any questions contact me at: davidevufficial@gmail.com


