import React, { useState, useEffect } from 'react';

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
	const [isPlaying, setIsPlaying] = useState(false);
	const [audioRef, setAudioRef] = useState(null);
	const [audioSrc, setAudioSrc] = useState(null);
	const [mediaSession] = useState(navigator.mediaSession);
	const readerlist = new FileReader();
	readerlist.readAsDataURL(file)
	readerlist.onload = (event) => {
		setAudioSrc(event.target.result);
	};
	useEffect(() => {
		if (mediaSession) {
			mediaSession.setActionHandler('play', () => setIsPlaying(true));
			mediaSession.setActionHandler('pause', () => setIsPlaying(false));
		}
	}, [mediaSession]);
	const title = file.name.split('.')[0];
	useEffect(() => {
		if (audioRef && mediaSession) {
			mediaSession.metadata = {
				title: title,
				artist: 'Unknown'
			};
		}
	}, [mediaSession]);
	const togglePlayback = () => {
		if (isPlaying) {
			audioRef.pause();
		} else {
			audioRef.play();
		}
	};
	return (
		<div>
			<audio ref={setAudioRef} controls="true" title={title}>
				<source src={audioSrc} type="audio/*" />
			</audio>
			<button onClick={togglePlayback}>{isPlaying ? 'Pause': "Play"}</button>
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
