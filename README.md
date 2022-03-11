# Mjolnir

> this is my first project in rust

> Mjolnir is chained data database inspired by block chain chaining block

## Conceptual :
![chain](https://user-images.githubusercontent.com/40946917/157789067-04038d27-dc65-4010-ace3-c78e7444308a.png)

> we using sha256 to hash data

> every box data is chained, will decrease fraud posibility
> i think is suitable for storing medical records, or ownership records or other data.
> data is a string, you can store json string to it


## How To install

### using homebrew :
  - add tap ``` brew tap alfiankan/mjolnir ```
  - then install ``` brew install mjolnir ```


## How to build from source
  - make sure you have rustup
  - clone this repository on master branch
  - then build ``` cargo build --all-targets --release ```
  - movemjolnir binary to bin directory ``` sudo cp target/release/mjolnir /usr/local/bin/ ```

## Terms
- Box : Box is imaginary block or Box conatins prev hash, data, and hash :
  ![box](https://user-images.githubusercontent.com/40946917/157790060-67213eb0-838d-4fc1-a69b-e66154d7b843.png)

- Genesis : genesis is process to make first Box with singularity hash, singularity hash is just random hash generated

- Chain : is relationship between box using hash<br>
  ![chainilustration](https://user-images.githubusercontent.com/40946917/157790117-bda71617-34a8-42c5-a8dc-83b822cb0671.png)

- record : is all data box with chain relationship from one genesis
  ![record](https://user-images.githubusercontent.com/40946917/157790217-9855de80-baf7-4ce0-985c-83be18bb2ad8.png)


## How to use
- make sure mjlnir installed
- create text file to store data
- set env ```export MJL_DATA_STORE=/path/to/mjfile/database.mj```
- open mjolnir cli by typing ```mjolnir``` in terminal

```
❯ mjolnir

	███╗░░░███╗░░░░░██╗░█████╗░██╗░░░░░███╗░░██╗██╗██████╗░
	████╗░████║░░░░░██║██╔══██╗██║░░░░░████╗░██║██║██╔══██╗
	██╔████╔██║░░░░░██║██║░░██║██║░░░░░██╔██╗██║██║██████╔╝
	██║╚██╔╝██║██╗░░██║██║░░██║██║░░░░░██║╚████║██║██╔══██╗
	██║░╚═╝░██║╚█████╔╝╚█████╔╝███████╗██║░╚███║██║██║░░██║
	╚═╝░░░░░╚═╝░╚════╝░░╚════╝░╚══════╝╚═╝░░╚══╝╚═╝╚═╝░░╚═╝
	============================================
	Version   : v1.0.0
	Home Page : https://github.com/alfiankan/mjolnir


[1] [mql] >
```

<br>

## Mjl (Mjolnir query language)


#### Genesis
 - to make new record you need genesis to do that type ```gen``` on mjolnir cli
 - then you get the key chain, save it
```
[1] [mql] > gen

   <> Generated Key Chain : 784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71
   <> Hint : Now you can insert new box
   <> Example : INSERT TO 784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71 'mystringdata'

[2] [mql] >
```

#### records
 - mjolnir support multiple records, to make another record just run genesis ```gen```
 - to list all records type ```records``` on mjolnir cli
```
[2] [mql] > records

   <> All Records :
   ================
   [1]	 1a48b9d63e7d3ce04e4fd570a2b6cb526ff3e07721ce14e8d50c1d86c3665f61

   [2]	 784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71

   ============================================================ Found : 2

[3] [mql] >
```

#### Insert
- to insert box (next box) you can do ```insert to <record key_chain> '<string data>'``` on mjolnir cli, make sure string data between single quotes ``` '<string_data>' ```

```
[6] [mql] > insert to 784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71 'owner:budi'

   <>	Box inserted to record with key chain 784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71
   [*]	prev 	: 2b9c08bc401d516139154cb74f46ad69384b3a35a717d37630100b8088e91937
   [*]	data 	: {"prev_hash":"2b9c08bc401d516139154cb74f46ad69384b3a35a717d37630100b8088e91937","data":"'owner:budi'"}
   [*]	hash 	: 83d7bc5bab8fd05bf5f7912be0039430ef8f698ab74150552c4a0d8fdc0599d6
```


#### Select
- to get last record data do ``` select <record key_chain> ```

```
[7] [mql] > select 784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71

	2b9c08bc401d516139154cb74f46ad69384b3a35a717d37630100b8088e91937
		    │
		   ─┴─
	{"prev_hash":"2b9c08bc401d516139154cb74f46ad69384b
	3a35a717d37630100b8088e91937","data":"'owner:budi'
	"}
		   ─┬─
		    │
	83d7bc5bab8fd05bf5f7912be0039430ef8f698ab74150552c4a0d8fdc0599d6
		    ▲
		    │
		    ▼

[8] [mql] >
```

#### Select All
- to get all box with chains record data do ``` select <record key_chain> all ```
```
[25] [mql] > select 784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71 all

	784168e6a916178a1701bfd01103f22df59a9fc3efd878ae86222f4636b51c71
		    │
		   ─┴─
	{"prev_hash":"784168e6a916178a1701bfd01103f22df59a
	9fc3efd878ae86222f4636b51c71","data":"{}"}
		   ─┬─
		    │
	916e3eb4f1566cd57336e64e2e0aed4b5d390e98492a91aa5c32b20608d58187
		    ▲
		    │
		    ▼
	916e3eb4f1566cd57336e64e2e0aed4b5d390e98492a91aa5c32b20608d58187
		    │
		   ─┴─
	{"prev_hash":"916e3eb4f1566cd57336e64e2e0aed4b5d39
	0e98492a91aa5c32b20608d58187","data":"'owner:josep
	h'"}
		   ─┬─
		    │
	bc20fa94f7c8826efb2c199e0457d65c3fa77fd42889782a75d153ca27687af7
		    ▲
		    │
		    ▼
	bc20fa94f7c8826efb2c199e0457d65c3fa77fd42889782a75d153ca27687af7
		    │
		   ─┴─
	{"prev_hash":"bc20fa94f7c8826efb2c199e0457d65c3fa7
	7fd42889782a75d153ca27687af7","data":"'owner:rahma
	t'"}
		   ─┬─
		    │
	2b9c08bc401d516139154cb74f46ad69384b3a35a717d37630100b8088e91937
		    ▲
		    │
		    ▼
	2b9c08bc401d516139154cb74f46ad69384b3a35a717d37630100b8088e91937
		    │
		   ─┴─
	{"prev_hash":"2b9c08bc401d516139154cb74f46ad69384b
	3a35a717d37630100b8088e91937","data":"'owner:budi'
	"}
		   ─┬─
		    │
	83d7bc5bab8fd05bf5f7912be0039430ef8f698ab74150552c4a0d8fdc0599d6
		    ▲
		    │
		    ▼
========== 4 valid from 4 box in chain ==========

[26] [mql] >
```

