<script lang="ts">
	let loading: boolean = $state(false);

	async function verifyMessage(): Promise<boolean> {
		loading = true;
		let userText: HTMLTextAreaElement | null = document.querySelector('#user-input');

		if (userText != null) {
			let header = new Headers();
			header.append('Content-Type', 'application/json');

			let response = await fetch('http://localhost:1337/optimessage/verify-message', {
				method: 'POST',
				body: JSON.stringify({ message: `${userText.value}` }),
				headers: header
			}).then((response) => {
				loading = false;
				return response.text();
			});

			console.log(response);
		}

		loading = false;
		return false;
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
		<p>
			{loading}
		</p>
	</div>
</form>
