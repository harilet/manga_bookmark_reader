<script lang="ts">
  import { open } from "@tauri-apps/plugin-shell";
  import dataStore from "./data_store";
  import ContextMenu from "./contextMenu.svelte";

  export let propValue: any;

  let borderClass = "";
  let reTrigger = false;

  $: {
    reTrigger = !reTrigger;
    setClasses();
  }

  async function setClasses() {
    if (await dataStore.get(getUrlWithoutChapter())) {
      borderClass = "warning-class";
    } else {
      borderClass = "normal-class";
    }
  }

  function openUrl() {
    open(propValue["url"]);
  }

  function searchForManga() {
    let name: string = propValue["name"];
    let updatedName = name
      .slice(0, name.indexOf(" Chapter"))
      .replaceAll(" ", "_");
    open("https://www.natomanga.com/search/story/" + updatedName);
  }

  async function setChapterWarning(event: any) {
    event.stopPropagation();

    let url = getUrlWithoutChapter();
    if (await dataStore.get(url)) {
      await dataStore.delete(url);
    } else {
      await dataStore.set(url, true);
    }
    await dataStore.save();
    reTrigger = !reTrigger;
  }

  function getUrlWithoutChapter() {
    return propValue["url"].split("/").slice(0, -1).join("/");
  }

  function ContextMenuHandler(event: any, sId: string) {
    switch (sId) {
      case "mark_issue":
        setChapterWarning(event);
        break;
      case "s-Natomanga":
        searchForManga();
        break;
      case "t-Manganato.gg":
      trySite("t-Manganato.gg");
      break;
      default:
        break;
    }
  }

  function trySite(site:string){
    if(site=="t-Manganato.gg"){
      nameToManganatoUrl()
    }
  }

  function nameToManganatoUrl()  {
    let name:string=propValue["name"];
    let end_pos = name.indexOf(" Chapter")

    let res_name1_temp: string = name.slice(0,end_pos);

    let res_name1="";
    res_name1_temp.split('').forEach(function(char){
      console.log(char);
      // if(char=='-' || char==' '){
        res_name1+=char;
      // }
    });
    
    // res_name1_temp=res_name1_temp.replaceAll("-","");
    // res_name1_temp=res_name1_temp.replaceAll(" ","");
    // res_name1_temp=res_name1_temp.replaceAll("/[0-9]/g","");
    console.log(res_name1)
    
    // .chars()
    //     .filter(|c| c.is_alphanumeric() || c == &'-' || c == &' ')
    //     .collect();
    // let res_name1 = res_name1_temp.replace(" ", "-").to_ascii_lowercase();

    // let name12 = &name[name.find("Chapter").unwrap()..];
    // let temp: Vec<_> = name12.match_indices(" ").collect();
    // let res_name2 = &name12[8..temp.get(1).unwrap().0]
    //     .replace(" ", "-")
    //     .replace(".", "-")
    //     .replace(":", "");

    // return format!(
    //     "https://www.manganato.gg/manga/{}/chapter-{}",
    //     res_name1, res_name2
    // );
}
</script>

<button on:click={openUrl} class="button-wrappper">
  <ContextMenu
    items={[
      { id: "mark_issue", name: "Mark issue" },
      { Group: [{ Search: [{ id: "s-Natomanga", name: "Natomanga" }]} ,
      {Try: [{ id: "t-Manganato.gg", name: "Manganato.gg" }]}
    ] },
    ]}
    onClick={ContextMenuHandler}
  >
    <div
      data-augmented-ui="
        tr-2-clip-x tl-2-clip-x br-2-clip-x
      "
      class="main-manga-area {borderClass} font-style"
    >
      <div class="index-date">
        <div class="name">
          {propValue["name"]}
        </div>
        <div class="date-time">
          {propValue["date_time"]}
        </div>
      </div>

      <div class="remaining-chapter">
        <div class="remaining-chapter-text">
          {propValue["remaining_chapters"]}
        </div>
      </div>
    </div>
  </ContextMenu>
</button>

<style>
  .button-wrappper {
    flex-grow: 1;
    padding: 0px;
    border: none;
  }
  .main-manga-area {
    border-radius: 5px;
    cursor: pointer;
    padding: 0px;
    padding-left: 5px;
    position: relative;
    background: transparent;
    text-align: start;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    border-radius: 0px;
  }

  .wrapper-class {
    padding: 2px;
    flex: auto;
  }

  .warning-class {
    color: var(--primary-color);
    border: solid 1px rgb(255 255 255 / 0.1);
    text-shadow:
      2px 2px 0 #000000,
      2px -2px 0 #000000,
      -2px 2px 0 #000000,
      -2px -2px 0 #000000,
      2px 0px 0 #000000,
      0px 2px 0 #000000,
      -2px 0px 0 #000000,
      0px -2px 0 #000000;
    background: repeating-linear-gradient(
      90deg,
      var(--primary-color),
      var(--primary-color) 1px,
      var(--background-color) 1px,
      var(--background-color) 3px
    );
  }

  .normal-class {
    color: var(--primary-color);
    border: solid 1px rgb(255 255 255 / 0.1);
    background: var(--background-color);
  }

  .index-date {
    display: flex;
    justify-content: space-between;
    margin-bottom: 5px;
    padding-right: 5px;
    margin-top: 20px;
    width: -webkit-fill-available;
  }

  .date-time {
    margin-top: 5px;
    color: gray;
    white-space: nowrap;
  }

  .name {
    margin-bottom: 10px;
    font-weight: 600;
    font-size: 105%;
    width: -webkit-fill-available;
  }

  .remaining-chapter {
    display: flex;
    justify-content: space-between;
    align-items: end;
    font-weight: 600;
    width: 100%;
  }

  .remaining-chapter-text {
    margin-bottom: 10px;
    color: gray;
  }
</style>
