<script>
  import { page } from '$app/stores';
  export let data;
  import DbgJsonData from "$lib/dbg/DbgJsonData.svelte";
  import Token from "$lib/components/Token.svelte";
  
  let lastHoveredOrth = '';
  let lastClickedOrth = '';
  const handleHover = event => {
    lastHoveredOrth = event.detail;
  };
  const handleClick = event => {
    lastClickedOrth = event.detail;
  };

  let tokenFormData = {
    orthography: '',
    lemma: '',
    definition: '',
    phonetic: '',
    status: '',
    notes: ''
  };

  function updateTokenFormData() {
    if (lastClickedOrth && data.tokens_dict[lastClickedOrth]) {
      tokenFormData = {...data.tokens_dict[lastClickedOrth]};
    }
  }
  $: lastClickedOrth, updateTokenFormData();

  async function createToken() {
    data.tokens_dict[lastClickedOrth] = {...tokenFormData};
    const token = data.tokens_dict[lastClickedOrth];

    const response = await fetch('http://127.0.0.1:3000/vocab/create_token', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        lang_id: token.lang_id,
        orthography: token.orthography,
        phonetic: token.phonetic,
        lemma: token.lemma,
        definition: token.definition,
        status: token.status,
        notes: token.notes
      }),
    });

    if (!response.ok) {
      const message = `An error has occurred: ${response.status}`;
      throw new Error(message);
    }

    const result = await response.json();
    console.log(result);
  }
  async function updateToken() {
    data.tokens_dict[lastClickedOrth] = {...tokenFormData};
    const token = data.tokens_dict[lastClickedOrth];

    const response = await fetch('http://127.0.0.1:3000/vocab/update_token', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        id: token.id.id?.String,
        lang_id: token.lang_id,
        orthography: token.orthography,
        phonetic: token.phonetic,
        lemma: token.lemma,
        definition: token.definition,
        status: token.status,
        notes: token.notes
      }),
    });

    if (!response.ok) {
      const message = `An error has occurred: ${response.status}`;
      throw new Error(message);
    }

    const result = await response.json();
    console.log(result);
  }

</script>

Page
<p>Text for language: `{$page.params.lang}`</p>
<p>File `{$page.params.file}`</p>
<hr>

<p>Title:</p>

<h1 class="font-bold">{data.metadata.title}</h1>

<p>Text area:</p>

<!-- <div class="p-4 leading-10 text-2xl">
  {#each data.tokens_strs as token}   
    <Token token={data.tokens_dict[token]} 
      on:token_hover={handleHover} 
      on:token_click={handleClick}
    />
  {/each}
</div> -->

<div class="p-4 leading-8 text-xl">
  {#each data.parsed_doc.constituents as sentence_constituent}   

    {#if sentence_constituent.type == 'Whitespace'}
    
      <span class="bg-green-200 whitespace-pre-wrap">{sentence_constituent.text}</span>
    
    {:else if sentence_constituent.type == 'Sentence'}
      <span class="border-1 border-blue-200 py-1 hover:bg-blue-200">

      <!-- <span class="bg-red-50 whitespace-pre-wrap"> -->
        {#each sentence_constituent.constituents as constituent}
        
          {#if constituent.type == 'CompositToken'}
          
            <Token token={data.tokens_dict[constituent?.text]} 
              on:token_hover={handleHover} 
              on:token_click={handleClick}
            />

          {:else if constituent.type == 'SubwordToken'}
                      
            <!-- <Token token={data.tokens_dict[constituent?.text]} 
              on:token_hover={handleHover} 
              on:token_click={handleClick}
            /> -->
            
          {:else if constituent.type == 'SingleToken'}
            
            <Token token={data.tokens_dict[constituent?.text]} 
              on:token_hover={handleHover} 
              on:token_click={handleClick}
            />

            
          {:else if constituent.type == 'Whitespace'}
            
            <span class="bg-green-200 whitespace-pre-wrap">{constituent.text}</span>
          
          {/if}

        {/each}
      <!-- </span> -->

      </span>
    {:else}
    
      <span class="">PANIC</span>
    
    {/if}

    

  {/each}
  <!-- {JSON.stringify(data.parsed_doc.constituents)} -->
</div>



<div class="p-4 bg-rose-50">
  <p>Last hovered: <b>{lastHoveredOrth}</b></p>
  <ol>
    <li>
      definition: {#if lastHoveredOrth != '' && data.tokens_dict[lastHoveredOrth]}
        <b>{data.tokens_dict[lastHoveredOrth].definition}</b>  
      {/if}
    </li>
    <li>
      phonetic: {#if lastHoveredOrth != '' && data.tokens_dict[lastHoveredOrth]}
        <b>{data.tokens_dict[lastHoveredOrth].phonetic}</b>  
      {/if}
    </li>
    <li>
      status: {#if lastHoveredOrth != '' && data.tokens_dict[lastHoveredOrth]}
        <b>{data.tokens_dict[lastHoveredOrth].status}</b>  
      {/if}
    </li>
  </ol>
</div>
<div class="p-4 bg-amber-50">
  <p>Last clicked: <b>{lastClickedOrth}</b></p>
  <ol>
    <li>
      definition: {#if lastClickedOrth != '' && data.tokens_dict[lastClickedOrth]}
        <b>{data.tokens_dict[lastClickedOrth].definition}</b>  
      {/if}
    </li>
    <li>
      phonetic: {#if lastClickedOrth != '' && data.tokens_dict[lastClickedOrth]}
        <b>{data.tokens_dict[lastClickedOrth].phonetic}</b>  
      {/if}
    </li>
    <li>
      status: {#if lastClickedOrth != '' && data.tokens_dict[lastClickedOrth]}
        <b>{data.tokens_dict[lastClickedOrth].status}</b>  
      {/if}
    </li>
  </ol>

  <div class="p-4 bg-amber-100">
    <p><b>Editor</b></p>
    {#if lastClickedOrth != ''}
      <form on:submit|preventDefault={data.tokens_dict[lastClickedOrth].id ? updateToken : createToken}>
        <label for="orthography">orthography:</label><br>
        <input class="border-solid border-2 border-gray-400" disabled type="text" id="orthography" bind:value={tokenFormData.orthography}><br>

        <label for="lemma">lemma:</label><br>
        <input class="border-solid border-2 border-gray-400" type="text" id="lemma" bind:value={tokenFormData.lemma}><br>

        <label for="definition">definition:</label><br>
        <input class="border-solid border-2 border-gray-400" type="text" id="definition" bind:value={tokenFormData.definition}><br>

        <label for="phonetic">phonetic:</label><br>
        <input class="border-solid border-2 border-gray-400" type="text" id="phonetic" bind:value={tokenFormData.phonetic}><br>
        
        <label for="status">status:</label><br>
        <select class="border-solid border-2 border-gray-400" id="status" bind:value={tokenFormData.status}>
          <option value="L1">L1</option>
          <option value="L2">L2</option>
          <option value="L3">L3</option>
          <option value="L4">L4</option>
          <option value="L5">L5</option>
          <option value="IGNORED">IGNORED</option>
        </select><br>

        <label for="notes">notes:</label><br>
        <textarea class="border-solid border-2 border-gray-400" id="notes" bind:value={tokenFormData.notes} /><br>


        <input class="mt-2 border-solid border-2 border-gray-400" type="submit" value="Update Token">
      </form>
    {/if}
  </div>

</div>




<!-- Text -->
<!-- <pre>
  {data.text}
</pre> -->

<DbgJsonData {data} />
<DbgJsonData name='tokenFormData bindings' data={tokenFormData} />


<style>
  span {
    /* white-space: break-spaces; */
    /* white-space: pre-wrap; */
  }
</style>