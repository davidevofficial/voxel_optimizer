# VoxelOptimizer
This program was made to optimize the meshes that are exported by **MagicaVoxel** (software made by Ephtracy), written in rust, it is the **fastest** and, thanks to some clever optimizations and tricks, it is also the **best** at compressing even if it is a **lossless compression**, which means that the quality of the mesh isn't traded with the speed of the execution of the program. If you tried to export a mesh using magicavoxel you would know that its mesh exporter is pretty inefficient and it is not ideal for gamedev. For this reason and also because I was unable to find a program such as this with the characteristics I had in mind, Voxel Optimizer was born. It is pretty similiar to an addon made for blender (Vox Cleaner V2 by Farhan) but it differs for many reasons:
1. Completely free
2. You don't need blender
3. Low ram usage
4. Low disk usage
5. Easy to use
6. multithreaded so that converting many models is blazingly fast

# Compatibility
Before explaining how it works I wanted to say that this program unfortunately only works for **windows**, if you know a little bit of rust you can contribute to other major platforms such as linux and mac. Also while this program doesn't use much cpu the better the cpu the faster will be the processes, the cpu also has to support multithreading to a certain capacity (Most of the cpu's will do the job). Last but not least to run this program you need a minimal amount of ram but it has to be at least as big as the models you are compressing.

# Usage and benchmarks
Watch this video: [Optimize magicavoxel models with voxel optimizer](https://www.youtube.com/watch?v=KspAgJy-C9A) for version 1.0 and this one: [A new version of VoxelOptimizer is out!!!](https://www.youtube.com/watch?v=8qdgk7juKSw) for version 2.0 or follow this instructions to get started.

**download the latest release (release v2.0.0)** and **extract it** in a folder or on your desktop, the important thing is that both the folder "src" and voxeloptimizer.exe are on the same directory (whether it is on the desktop or in another folder).

To run the program **double click** onto the executable and two windows will open.
You'll have something like this:![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/851990a9-ab26-4c67-b456-c701035e5b01)


If you are on version 2.0 you can **drag and drop** every project file (.vox) you want to convert or use .ply files exported from magicavoxel itself. If you plan on using .vox files then you can export an obj with different materials depending on your settings.

If you are on version 1.0.1 to convert models to an optimized and superior form you first have to create the models in magicavoxel and export them using the second option (.ply) and then **drag and drop** every file you want to convert (you can and it's better if you do more than one at the time, tip: Control + A selects all the files in a folder) like so:

https://github.com/davidevofficial/voxel_optimizer/assets/127616649/4568ff63-293d-4748-83a3-ced18711c548

The default options are the best if you care about output file size, however depending on your needs you might need to change some, so here is every setting and its explanation with pros and cons.

## Algorithm Options

This settings influence the algorithm used to optimize the models.

### Cross-overlapping optimization

This setting changes the way the algorithm works while reducing the amount of vertices, to explain how it works here are some examples:

Let's say you have a cross:![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/6ce2e925-1da8-4e6e-8920-63c1ae0c1d8a)

| Off | On |
| ---- | ---|
|Without the option it would be divided like so: | With the option the green and blue part become united: |
|![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/1b3778b5-1a3c-4683-b68a-49827089e208) | ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/1c030205-fb38-41de-a3a8-8a0dc7afcd33) |
| (3 cubes) | (2 overlapping cubes)


**Reccomended: ON**

**Pros**:
1. Reduces File Size

**Cons**:
1. Slightly slower
2. The cubes overlap generating too much overdraw (in some software this results in bad behaviour)

### Enable solid colours to be one pixel on the texture map

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

**Reccomended: ON**

**Pros**:
1. Greatly reduces File Size
2. Can use the next setting (pattern matching) at its fullest

**Cons**:
1. Slightly slower
2. Cannot manually modify the texture of the face since if you modify a pixel you modify all the face

### Pattern matching

If it is on each texture will be flipped, rotated (in every way possible) and then compared to each other if two are equal than both faces will share the same region on the texture map:

![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/59005f60-1d81-4b44-8a69-6366196cb5ef)


**Recommended:ON**

**Pros**:
1. Greatly reduces File Size

**Cons**:
1. Anything higher than 0 makes it way slower
2. Cannot manually modify the texture of a face without modifying the texture of all the faces equal to that one.

### Let Glass be more accurate

This setting only works when you have materials (.vox files only), if there is glass then it generates more faces:

![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/f9f47b87-b92c-4c69-a379-b6b4e9c4b8e0)


**Reccomended: OFF**

**Pros**:
1. Glass has correct behaviour

**Cons**:
1. Increased file size
2. Slightly slower

## Export Options

This settings influence the way the mesh is exported

### Enable manual settings of the precision levels

"precisions level" is the amount of digits after the dot in the output .obj file for each vt; in this image that number is 3: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/96b4f7d4-b264-480e-8f6a-3b5457c23ab6)

This is what happens if the digits precision is incorrect:

| 2 Digits precision | 3 Digits precision |
| ---- | ---|
| ![2digitsprecision](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/73853f03-5988-4b30-ab56-184d6df15099) | ![3digitsprecision](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/6d5a41c3-7453-4e7a-b157-c09c6a3b921f) |

If the setting is off the program automatically detects the amount of digits otherwise you can specify it yourself.

By default:

|Width/Height       | Digits after the dot | y*      |
| ----------------- | -------------------- | ------- |
| 1                 | 0                    | 0       |
| 2                 | 1                    | 1       |
| 4                 | 2                    | 2       |
| 8                 | 3                    | 3       |
| 16                | 4                    | 4       |
| 32                | 4                    | 5       |
| 64                | 4                    | 6       |
| 128               | 5                    | 7       |
| 256               | 5                    | 8       |
| 512               | 5                    | 9       |
| 1024              | 6                    | 10      |
| 2048              | 6                    | 11      |
| 4096 <=x <=8192   | 6                    | log₂(x) |
| 16384 <=x <=65536 | 7                    | log₂(x) |

y*: Digits needed to write the full number without approximations (useful if you want the max precision without adding useless digits)
**: The program works only with powers of 2 for textures

**Reccomended: OFF**

**Pros if it is on**:
1. Manually set digits numbers
2. more control
3. Potentially more high quality

**Pro if it is off**:
1. You don't have to manually set digits numbers

### Background Colour

Defines the colour of the pixels not used but present in the texture map.

How to use it: If you have a small palette it can save a really small amount of disk space if you use as a background the same colour as one present in the palette.

**Reccomended: OFF (doesn't really matter that much)/The same as the most used colour in the model**

### Coordinate system

Based on the software you need to export to you may need to change the coordinate system, follow the table below:
![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/9c5fa9d9-6584-4475-af6d-90826c0d9a98)

**Reccomended:**

| Software        | Y-UP   | Right-handed |
|-----------------|--------|--------------|
| Blender         | False  | True         |
| Unreal engine   | False  | False        |
| Godot           | True   | True         |
| Unity           | True   | False        |

### Origin is the center of the model

If you select it the model vertices will not have their position based on their magicavoxel position.

| off | on |
| ---- | ---|
|![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/cfd05e92-1fe6-4a07-9ed0-90f1440caaee) | ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/78acbc1f-142d-44a0-b99a-94137e6b1325)


**Pros**:
1. 2 meshes will have their positions relative one to the other when importing the mesh in other programs

**Cons**:
1. Consumes a really tiny amount of disk space (especially if there are many small models created all over the place in magicavoxel)

### Enable Normals in the final export

If required by the software you need to export to then activate this setting.

If you notice weird lightning on the mesh then activate this setting.

**Pros**:
1. The mesh is more accurate.

**Cons**:
1. Consumes more disk space (6 bytes per face + 60 bytes to be exact).


### Enable UV debug mode

If you enable this all models will have this texture ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/08db6b00-7758-45e3-a3a6-7b16e51b10d5)

correctly applied to each face.

### Enable Custom export scale for the Model

If you enable this option three sliders will appear, the first represent a dilation of the final mesh on the X axis while the second one on the Y axis and the third one on the Z axis.

When writing to file vertices follow this format: "v X Y Z" when the size of the model is even (x: 2n, n∈ℝ), if it is odd (x: 2n+1, n∈ℝ) it follows this format: "v X.5 Y.5 Z.5" however when a custom scale is selected (for instance 0.1X, 0.1Y, 0.1Z) it becomes v X.x Y.y Z.z (increasing size on disk),
however if and only if an odd integer number is selected then it always becomes  "v X Y Z" thus decreasing size on disk

*Pros*:

1. You can modify the Scale of the object before importing inside of other programs.
2. You don't have to modify the scale for each mesh on an external program
3. Might decrease Disk size

*Cons*:

1. Might Increase Disk size


## Compatibility options

If it is not ON you might encounter meshes that appear to be correct but are way too big then what they need to be

### Enable de-cull optimization

The ply magicavoxel optimizes meshes when exporting such that a cube full inside is actually a cube empty inside but since you can't see it it doesn't matter except that it does if you have to compress it with this program.

**Pros**:
1. Greatly reduces File Size

**Cons**:
1. Slightly slower

Reccomended: ON


### Exable UV extra precision

Some programs requires this setting to be on for the model to be displayed correctly because they need very precise UVs

| Off | On | Godot Setting |
| ---- | ---| -------------|
| ![before](https://github.com/user-attachments/assets/7ee1519b-5a6a-4baa-adc1-83768a2f3fd0) | ![after](https://github.com/user-attachments/assets/cb087f76-5b3c-4720-8f16-3157440d3e6b) | ![fix](https://github.com/user-attachments/assets/3af8d510-4cde-4e36-a499-a25b7ed892d7) |



*Pros*:

1. More Compatible or "Correct" meshes

*Cons*:

1. Increases disk size because each Vt (see .obj documentation) holds 1 texture instead of being shared by 4 textures, increasing Vt count by at most a factor of 4

Reccomended: OFF (unless there are artifacts in your mesh)

## .VOX specific options

These only matter when using .vox files

### All the models in one file

As the name sugggests it puts all the models in one big .obj, optimizing the output while doing so

If this is unselected the program will convert each nSHP (each model) utilizing threads (in parallel so it is faster). A new option will appear called *detailed export name* which chnges the name of the output by including the group the nSHP belongs to.

The output will be {FileName}_{GroupName if detailed export name is enabled}_{ObjName}_{CHUNK_ID}

**Reccomended:ON**

**Pros:**
1. Smaller overall size

**Cons:**
1. Increased RAM usage while optimizing
2. Can't modify or use singular models because they are now all part of one

### Transparency, Emission, Roughness, Metallic, IOR and Specular Maps

If all of these are ON this is what the .mtl looks like (Given a .vox project called GlassTest): ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/9654d308-169c-4ba2-ac4a-b17a07fcf23a)

and this is what the file structure looks like: ![image](https://github.com/davidevofficial/voxel_optimizer/assets/127616649/efa02adc-644e-4ed6-9088-cc3a362e3f5b)


**WARNING:**
You might want to modify manually the emission map with another program (I'd suggest [Slate](https://github.com/mitchcurtis/slate/releases/tag/v0.9.0), just download, extract, load \_emit.png, click ctrl + A, go to **image>adjustments>Hue/Saturation**, modify, click ctrl + S) and the Alpha of the Albedo Map to make glass look more dense (You can do this as by changing The emission map but instead of hue/saturation you change opacity, for slate click the two checkbox and increase the slider).

**Reccomended: Depends on your needs, overall only Transparency and Emission is fine**


## Convert

After the settings you should choose a directory where the output will be written to and then click the convert button. Once you are finished you are free to close the program.
The program should notify you when it finishes, if it doesn't move the mouse or if too much time has passed retry but using the command prompt version of the software which you can find here: [itch.io](https://davidevofficial.itch.io/voxeloptimizer) and if there is a panic message share the logs to me (davidevuffical@gmail.com).


## Benchmarks

To benchmark I'll use the .vox files that magicavoxel comes by default and I'll compare the .obj of magicavoxel with the .obj of my program

MV = magicavoxel, VO = voxeloptimizer

| Model Name            | MV Export size | MV Export speed | VO Export Size  | VO Export speed |
|-----------------------|----------------|-----------------|-----------------|-----------------|
|3x3x3                  |4.41kb          |N/A              |2.73kb           |73ms             |
|Lightsabers            |14.8kb          |N/A              |7.4kb            |98ms             |
|Castle+Chr_knight+Cars |276.2kb         |N/A              |126.5kb          |485ms            |
|Doom                   |632.1kb         |N/A              |84.9kb           |473ms            |
|Teapot                 |2.82mb          |About 3s         |1.27mb           |2.92s            |
|Menger                 |19.5mb          |About 7s         |8.33mb           |22.93s           |

VoxelOptimizer clearly beats Magicavoxel out of the water it is about two times as disk efficient (without accounting for the fact that they contain about 10x less faces) while unfortunately being slightly slower than Magicavoxel, VoxelOptimizer has also other advantages:
|                                         |MV |VO |
|-----------------------------------------|---|---|
|Converting multiple files at once        |No |Yes|
|Exporting to one file                    |No |Yes|
|Exporting materials                      |No |Yes|
|Various coordinate systems to choose from|No |Yes|
|Normals                                  |Yes|Yes|
|Optimized for gamedev                    |No |Ye!|


In the benchmark folder of this repository you can find all of the data (.vox files, .ply files, my output, magicavoxel output).


# License and contributions
I would be glad for any pull request, discussion, issues you have to make this program better.

License: you may modify and copy for private use the software but you cannot redistribute or sell it.

If you have any questions contact me at: davidevufficial@gmail.com


