import React, { useEffect, useState } from 'react'
import {collection, DocumentData, onSnapshot} from 'firebase/firestore'
import {Movie, MovieType} from '../typings'
import {firestore as db} from '../firebase'
function use_list(uuid: string | undefined) {

  const [list, setList] = useState<MovieType[] | DocumentData[]>([]);

  useEffect(() => {
    if  (!uuid) return 

    return onSnapshot(
      //  This will give us all the documents which we will store 
      //  inside of our useState
      collection(db, "Customers", uuid, "List1"),
      (snapshot) => { 
        setList(snapshot.docs.map((doc) => ({ 
          id: doc.id,
          ...doc.data,
        })))
      })
  }, [db, uuid])
  


  return list 
}

export default use_list

