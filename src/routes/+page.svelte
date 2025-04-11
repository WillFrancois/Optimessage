<script lang="ts">
	let loading: string = $state('');

	async function verifyMessage(): Promise<boolean> {
		if (loading === 'Moderating...') {
			console.log('Still loading!');
			return false;
		}
		loading = 'Moderating...';
		let userText: HTMLTextAreaElement | null = document.querySelector('#user-input');

		if (userText != null) {
			let header = new Headers();
			header.append('Content-Type', 'application/json');

			let response = await fetch('http://localhost:1337/optimessage/verify-message', {
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
			class="resize-none place-self-center md:w-2/1 lg:w-3/1"
		></textarea>
		<button
			class="mt-1 place-self-center border-1 p-1 md:w-1/2 lg:w-3/4"
			type="submit"
			aria-label="Submit Button"
			onclick={verifyMessage}>Submit</button
		>
		<p class="text-center">
			{loading}
		</p>
	</div>
</form>
