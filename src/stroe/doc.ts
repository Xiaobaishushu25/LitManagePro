import {defineStore} from "pinia";
import {ref} from "vue";
import {DocumentTags} from "../components/main/main-type.ts";

const useDocStore = defineStore('docs', ()=>{
    const docs = ref<DocumentTags[]|null>(null);
    const currentSelectDoc = ref<DocumentTags>();
    //不知道为啥这两个watch没用，留这以后再看
    // watch(()=> docs.value, (newValue)=>{
    //     console.log("data改变了")
    //     currentSelectDoc.value = newValue?.[0];
    // })
    // watchEffect(()=>{
    //     console.log("currentSelectDoc改变了")
    //     currentSelectDoc.value = docs.value?.[0];
    // })
    function addNewDoc(doc: DocumentTags) {
        docs.value!.push(doc);
    }
    function setAllDocs(_docs: DocumentTags[]) {
        docs.value = _docs;
        currentSelectDoc.value = _docs[0];
    }
    function setCurrentSelectDoc(doc: DocumentTags) {
        currentSelectDoc.value = doc;
    }
    function updateDoc(doc: DocumentTags){
        docs.value!.forEach((item,index)=>{
            if(item.id === doc.id){
                docs.value![index] = doc;
            }
        })
        if (currentSelectDoc.value?.id === doc.id){
            currentSelectDoc.value = doc;
        }
    }
    function deleteDocs(ids: number[]){
        console.log(ids)
        console.log(docs.value!.length)
        docs.value = docs.value!.filter(doc => !ids.includes(doc.id));
        console.log(docs.value!.length)
        if (ids.includes(currentSelectDoc.value!.id)){
            currentSelectDoc.value = undefined;
        }
    }
    return {
        docs,
        currentSelectDoc,
        setAllDocs,
        setCurrentSelectDoc,
        addNewDoc,
        updateDoc,
        deleteDocs
    }
})
export default useDocStore