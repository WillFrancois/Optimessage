<script lang="ts">
	function verifyMessage(): boolean {
		let userText: HTMLTextAreaElement | null = document.querySelector('#user-input');

		if (userText != null) {
			console.log(userText.value);
		}

		return false;
	}
</script>

<h1 class="text-center">Welcome to Optimessage!</h1>

<form>
	<div class="grid w-screen place-content-center">
		<h2 class="mt-5 text-center">Write something nice:</h2>
		<textarea id="user-input" class="resize-none place-self-center md:w-2/1 lg:w-3/1"></textarea>
		<button
			class="mt-1 place-self-center border-1 p-1 md:w-1/2 lg:w-3/4"
			type="submit"
			aria-label="Submit Button"
			on:click={verifyMessage}>Submit</button
		>
	</div>
</form>
