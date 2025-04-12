<script lang="ts">
	interface PageProps {
		data: {
			messageList: string[];
		};
	}

	let { data }: PageProps = $props();
	let loading: string = $state('');

	async function verifyMessage(): Promise<boolean> {
		let userText: HTMLTextAreaElement | null = document.querySelector('#user-input');

		if (
			loading ===
			'Thank you for your submission! This will be posted if it passes the automated moderating process.'
		) {
			console.log('Still loading!');
			return false;
		}
		loading =
			'Thank you for your submission! This will be posted if it passes the automated moderating process.';

		if (userText != null) {
			if (userText.value === '') {
				loading = 'Please provide a value before submitting!';
				return false;
			}

			let header = new Headers();
			header.append('Content-Type', 'application/json');

			let response = await fetch('http://localhost:1337/optimessage/verify-message', {
				method: 'POST',
				body: JSON.stringify({ message: `${userText.value}`, createdAt: Date.now() }),
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

<h1 class="text-center text-[2rem]">Welcome to Optimessage!</h1>

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
	<h2 class="mt-7 text-center text-[1.2rem]">Recent messages:</h2>
	<ul>
		{#each data.messageList as message}
			<div class="m-5 h-15 place-content-center bg-blue-50">
				<li><strong><p class="text-center">{message}</p></strong></li>
			</div>
		{/each}
	</ul>
</form>
