<script lang="ts">
	let loading: string = $state('');

	async function verifyMessage(): Promise<boolean> {
		if (
			loading ===
			'Thank you for your submission! This will be posted if it passes the automated moderating process.'
		) {
			console.log('Still loading!');
			return false;
		}
		loading =
			'Thank you for your submission! This will be posted if it passes the automated moderating process.';
		let userText: HTMLTextAreaElement | null = document.querySelector('#user-input');

		if (userText != null) {
			let header = new Headers();
			header.append('Content-Type', 'application/json');

			let response = await fetch('http://localhost:1337/optimessage/verify-message', {
				mode: 'no-cors',
				method: 'POST',
				body: JSON.stringify({ message: `${userText.value}` }),
				headers: header
			}).then((response) => {
				return response.text();
			});

			console.log(response);
			if (response === 'true') {
				loading = 'Submitted successfully!';
			} else if (response === 'false') {
				loading = 'This comment was deemed unsuitable.';
			} else if (response === 'unknown') {
				loading = "You've confused the moderator. Please try again. :(";
			} else if (response === 'busy') {
				loading = 'Server is busy. Try again later!';
			} else {
				loading = "Something's gone wrong. Please try reloading the page and submitting again.";
			}
		}

		return true;
	}
</script>

<h1 class="text-center">Welcome to Optimessage!</h1>

<form>
	<div class="grid w-screen place-content-center">
		<h2 class="mt-5 text-center">Write something nice (100 characters max):</h2>
		<textarea
			maxlength="100"
			id="user-input"
			class="shadow-black-100 resize-none place-self-center rounded-full shadow-lg md:w-1/1 lg:w-3/1"
		></textarea>
		<button
			class="shadow-black-100 mt-1 place-self-center rounded-full border-1 p-1 shadow-lg md:w-1/2 lg:w-3/4"
			type="submit"
			aria-label="Submit Button"
			onclick={verifyMessage}>Submit</button
		>
	</div>
	<p class="text-center">
		{loading}
	</p>
</form>
