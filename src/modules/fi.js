import React, { useState, useEffect } from 'react';
import AudioPlayer from 'react-h5-audio-player';
import { SlControlPlay } from "react-icons/sl";
import { SlControlPause } from "react-icons/sl";
import { SlControlForward } from "react-icons/sl";
import { SlControlRewind } from "react-icons/sl";
import { SlControlStart } from "react-icons/sl";
import { SlControlEnd } from "react-icons/sl";

function Um({ isUnderDevelopment}) {
	if (isUnderDevelopment) {
		return (
			<h1>
				Don't Work cus under development
			</h1>
		);
	};
	return null;
};

function LoadFile({ file }) {
	if (file == null) {
		return null;
	};
	const [audioSrc, setAudioSrc] = useState(null);
	const readerlist = new FileReader();
	readerlist.readAsDataURL(file)
	readerlist.onload = (event) => {
		setAudioSrc(event.target.result);
	};
	return (
		<div>
			<AudioPlayer
				autoPlay
				src={audioSrc}
				custonIcons={{
					play: SlControlPlay,
					pause: SlControlPause,
					forward: SlControlForward,
					rewind: SlControlRewind
				}}
			/>
		</div>
	);
};

function Mus({ file }) {
	if (file != null) {
		return (
			<div>
				<p>{file.name.split('.')[0]}</p>
				<LoadFile file={file} />
			</div>
		);
	};
	return null;
};

function FileInput() {
	const [sus, setSus] = useState(false);
	const [selectedFile, setSelectedFile] = useState(null);

	const handleFileChange = (event) => {
		for (let f = 0; f < event.target.files.length; f++) {
			const file = event.target.files[f]
			if (!file || !file.type.startsWith('audio/')) {
				alert('Use audio file');
				return;
			}
			setSelectedFile(file);
		};
	};

	const UrlLoad = (event) => {
		if (event.key === 'Enter') {
				const art = document.getElementById("Artist").value;
				const tit = document.getElementById("Title").value;
				const key = document.getElementById("apiKey").value;
				setSus(true);
		};
	};
	return (
		<div>
			<input type="file" accept="audio/*" onChange={handleFileChange} />
			<input type="text" id="Artist" placeholder="Artist" onKeyPress={UrlLoad} />
			<input type="text" id="Title" placeholder="Title" onKeyPress={UrlLoad} />
			<input type="text" id="apiKey" placeholder="Api Key" onKeyPress={UrlLoad} />
			<Um isUnderDevelopment={sus} />
			<Mus file={selectedFile} />
		</div>
	);
};

export default FileInput;
