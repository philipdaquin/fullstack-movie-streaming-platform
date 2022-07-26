import Head from 'next/head'
import Link from 'next/link'
import React from 'react'
import useAuth from '../hooks/useAuth'
import Image from 'next/image'
import useSubscription from '../hooks/useSubscription'
import { User } from 'firebase/auth'
import {UserIcon} from '@heroicons/react/outline'
import UserMembership from '../components/user_memsbership'
import { getProducts, Product } from '@stripe/firestore-stripe-payments'
import payments from '../lib/stripe'
import { GetStaticProps } from 'next'
import  Header from '../components/header'

interface AccountProps { 
  products: Product[]
}

function Account({products}: AccountProps) {
  const {user, log_out} = useAuth();
  const subscription = useSubscription(user);

  console.log(products);


  return (
    <div>
      <Head>
        <title>Account Settings</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Header/>
      <main className='pt-28 mx-auto px-5 max-w-7xl pb-12 transition-all md:px-10'>
        <div className='flex space-x-4'>
          <h1 className='text-3xl mb:4xl font-medium'>My Account</h1>
          <div className='flex items-center gap-x-1.5 '>
            <img src="https://rb.gy/4vfk4r" alt=""  className='h-6 w-6'/>
            <p className='text-xs font-semibold text-white-200'>Member since {subscription?.created}</p>
          </div>
        </div>


        <UserMembership />


        <div className='items-center'>
            <div className='mt-6 mb-5 grid grid-col-4 gap-x-5 border border-white p-5 md:grid-cols-4 md:border-x-0 md:border-t 
              md:border-b-0 md:px-0 md:pb-0'>
              <h4>Plan Details: </h4>
              <div className='cols-2 font-regular'>{
                products.filter((plan) => plan.id === subscription?.product)[0]?.name
                }
              </div>
              <div className='flex justify-end col-end-5'>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Change plan</p>
              </div>
            </div>
          </div>
          <div className='mt-2 grid grid-cols-1 gap-x-4 border border-white p-4  md:grid-cols-4 md:border-x-0 md:border-t
            md:border-b-0 md:px-0 md:pb-0'>
              <h4>Settings: </h4> 
              <ul>
                <li>
                  <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Communication settings</p>
                </li>
                <li>
                  <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Marketing communications</p>
                </li>
                <li>
                  <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Parental controls</p>
                </li>
                <li>
                  <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Test participation</p>
                </li>
                <li>
                  <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Activate a device</p>
                </li>
                <li>
                  <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Recent device streaming activity</p>
                </li>
                <li>
                  <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Sign out of all devices</p>
                </li>
              </ul>
          </div>
          <div className='mt-6 grid  gap-x-4 border border-white p-5  md:grid-cols-4 md:border-x-0 md:border-t
            md:border-b-0 md:px-0 md:pb-0'>
            <h4>My Profile</h4>
            <ul className=''>
              <li>
                <div className='pb-4 flex space-x-3 items-center'>
                  <img 
                    src="https://rb.gy/g1pwyx" 
                    alt="" 
                    className='cursor-pointer rounded '
                  />
                  <p className='font-regular text-xl'>{user?.displayName || user?.email}</p>
                </div>
              </li>
              <li>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Language</p></li>
              <li>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Playback settings</p></li>
              <li>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Subtitle Appearance</p></li>
              <li>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Viewing Activity</p></li>
              <li>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Ratings</p></li>
              <li>
                <p className='hover:underline text-blue-500 text-sm cursor-pointer'
                  onClick={log_out}>
                  Sign out of all devices
                </p>
              </li>
            </ul>

            <div className='flex justify-end col-end-5 col-span-2'>
              <p className='hover:underline text-blue-500 text-sm cursor-pointer'>Manage Profiles</p>
            </div>
          </div>
      </main>
    </div>
  )
}

export default Account
// This is also found in the Index File!
export const getStaticProps: GetStaticProps = async () =>  { 
  const products = await getProducts(payments, { 
    includePrices: true,
    activeOnly: true
  })
    .then((resp) => resp)
    .catch((e) => console.log(e.message));
  return { 
    props: { 
      products
    }
  }
}