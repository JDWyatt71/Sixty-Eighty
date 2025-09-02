# Sixty-Eighty

Description:
This is a command-line database system for cataloguing music artists and their albums, specifically those active during the years 1960-1980. This period was chosen as I believe it was a very interesting and diverse period in popular music which saw a great deal of evolution across its span. The program was coded in Rust.

Features:
-Adding, editing, and deleting of albums and artists
-Creating and editing album rankings for each artist
-Printing information regarding albums, artists and rankings, including printing all albums/artists in the system, printing all fields of an album/artist, and printing an overall timeline showing all albums by release year
-Serialisation of added data to text files so additions are permanent. This is done when inputting the commands 'ex' and 'save'. Data is not saved if program is closed through other means.

Usage:
To run the program, go to target -> release then run main.exe. If all of the txt files in this folder are removed/deleted the project will recreate them when the next save occurs. Please do not delete one or two of these files as this will cause errors.

Known issues:
-The final year on a line of timeline will overrun onto the line below if it is over 11 characters. I intend to fix this issue at a later date but doing so will require restructuring the timeline function, and as it is a relatively minor visual bug I have decided to leave it in.
-The timeline also cannot handle multiple albums from the same artist in the same year. While this isn't ideal it would be a lot of effort to make this work so instead I have decided to print any missed albums below the timeline itself.
-There are few entries loaded into the system by default; as I wanted to release this project quickly, I abstained from populating the release version with a large number of default entries. I plan on adding more in a future update.

This is an open source project, feel free to download, use, and edit the source code.

