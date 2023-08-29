<script>
    let counter = 69;
</script>

I'm counting... {counter}
<button
    on:click={() => {
        counter++;
    }}>Count up</button
>

<style>
    button {
        background: green;
        color: #fff;
    }
</style>
