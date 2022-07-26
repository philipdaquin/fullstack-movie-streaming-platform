import Head from 'next/head'
import Link from 'next/link'
import React, { useState } from 'react'
import Image from 'next/image'
import useAuth from '../hooks/useAuth'
import {CheckIcon} from '@heroicons/react/outline'
import { NEXT_REQUEST_META } from 'next/dist/server/request-meta'
import { Product } from '@stripe/firestore-stripe-payments'
import Table from './table'
import { CircularProgress } from '@mui/material'
import { load_checkout } from '../lib/stripe'
// import Loader from './loader'

interface PlanProps { 
    products: Product[]
}

function Plans({products}: PlanProps) {

  const { log_out, user } = useAuth();
  // Default Product is Premium Membership
  const [membership_plan, SetMembershipPlan] = useState<Product | null>(products[2]);
  const [billLoading, setBillLoading] = useState(false);

  const subscribeToPlan = () =>  {
    //  If no user return nothing
    if (!user) { return }

    load_checkout(membership_plan?.prices[0].id!);
    setBillLoading(true);

  }
  
  return (
    <div className=''>
        <Head>
            <title>Netflix - Subscribe Now!</title>
            <link rel="icon" href="/favicon.ico" />
        </Head>

        <header className='border-white/10 bg-[#141414'>
            <Link href="/">
                <img 
                src="https://upload.wikimedia.org/wikipedia/commons/0/08/Netflix_2015_logo.svg" 
                alt="" 
                width={100}
                height={100}
                className="cursor-pointer, object-contain"
                />
            </Link>
            <button
                onClick={log_out} 
                className='text-lg font-medium hover:underline'>Sign Out</button>
        </header>
        <main className='pt-28 px-5 pb-12 mx-auto max-w-5xl transition-all md:px-10'>
            <h1 className='text-3xl mb-3 font-medium flex '>Choose the plan that's right for your</h1>
            <div>
                <ul>
                    <li className='flex items-center gap-x-2 text-lg'>
                        <CheckIcon className="h-7 w-7 text-red-600"/> Watch all you want. Ad-Free
                    </li>
                    <li className='flex items-center gap-x-2 text-lg'>
                        <CheckIcon className="h-7 w-7 text-red-600"/> Recommendations just for you.
                    </li>
                    <li className='flex items-center gap-x-2 text-lg'>
                        <CheckIcon className="h-7 w-7 text-red-600"/> Change or cancel your plan anytime
                    </li>
                </ul>
                <section className="mt-4 flex flex-col space-y-4 ">
                    <article className='flex w-full items-center justify-end self-end md:w-3/5 pt-5'>
                        {products.map((product) => ( 
                            <div key={product.id} className={ 
                                `plan__box ${membership_plan?.id === product.id ? 'opacity-100' :  'opacity-60'
                            }`} 
                                onClick={() => SetMembershipPlan(product)}
                            >
                                {product.name}
                            </div>
                        ))}
                    </article>
                    <Table products={products} selected_plan={membership_plan? membership_plan : null}/>
                    <div className='flex justify-center pt-7'>
                        <button 
                            onClick={subscribeToPlan}
                            disabled={!membership_plan || billLoading}
                            className={`p-5 bg-red-600 rounded w-[50%] font-semibold text-2xl 
                                ${billLoading && 'opacity-60'}`
                            }>
                            {
                                billLoading ? ( 
                                    // <Loader color="dark:fill-gray-300"/>
                                    <CircularProgress color='inherit'/>
                                ): ( 
                                    'Subscribe' 
                                )
                            }
                        </button>
                    </div>
                </section>
            </div>            
        </main>
    </div>
  )
}

export default Plans

