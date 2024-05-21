import React, { useState } from 'react';

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
			fLiHandle(file);
		};
	};
	const fLiHandle = (f) => {
			const readerlist = new FileReader();
			readerlist.readAsDataURL(f)

			readerlist.onload = (event) => {
				const nfile = document.createElement('audio');
				nfile.setAttribute('src', event.target.result);
				nfile.controls = true;

				const pr = document.createElement('div');
				const label = document.createElement('p');
				const lab = document.createTextNode(f.name)
				label.appendChild(lab);
				pr.appendChild(label)
				pr.appendChild(nfile);
				document.body.appendChild(pr);
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
			<input type="file" multiple accept="audio/*" onChange={handleFileChange} />
			<input type="text" id="Artist" placeholder="Artist" onKeyPress={UrlLoad} />
			<input type="text" id="Title" placeholder="Title" onKeyPress={UrlLoad} />
			<input type="text" id="apiKey" placeholder="Api Key" onKeyPress={UrlLoad} />
			<Um isUnderDevelopment={sus} />
		</div>
	);
}

export default FileInput;
