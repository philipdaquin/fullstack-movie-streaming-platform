import { CheckIcon, XIcon } from '@heroicons/react/outline'
import { Product } from '@stripe/firestore-stripe-payments'
import React from 'react'

interface MetaDataProps { 
    products: Product[],
    selected_plan: Product | null
}

function Table({products, selected_plan}: MetaDataProps) {
  return (
    <table>
        {/* DivideY adds Hr */}
        <tbody className='divide-y divide-[gray]'>
            <tr className='table__row'>
                    <td className='table__data__title'>Monthly Price</td>
                        {products.map((product) => ( 
                            <td key={product.id} className={`table__data ${selected_plan?.id === product.id ? 'text-[#E40912]' : 'text-[gray]'}`}>
                            $AUD{product.prices[0].unit_amount! / 100}
                        </td>
                        ))}
                    <td>
                </td>
            </tr>
            <tr className='table__row'>
                    <td className='table__data__title'>Video Quality</td>
                        {products.map((product) => ( 
                            <td key={product.id} className={`table__data ${selected_plan?.id === product.id ? 'text-[#E40912]' : 'text-[gray]'}`}>{product.metadata.video_quality}</td>
                        ))}
                    <td>
                </td>
            </tr>

            <tr className='table__row'>
                    <td className='table__data__title'>Resolution</td>
                        {products.map((product) => ( 
                            <td key={product.id} className={`table__data ${selected_plan?.id === product.id ? 'text-[#E40912]' : 'text-[gray]'}`}>{product.metadata.resolution}</td>
                        ))}
                    <td>
                </td>
            </tr>
            <tr className='table__row'>
                    <td className='table__data__title'>Watch on your TV, computer, mobile phone and tablet</td>
                        {products.map((product) => ( 
                            <td key={product.id} className={`table__data ${selected_plan?.id === product.id ? 'text-[#E40912]' : 'text-[gray]'}`}>{
                                product.metadata.portability === 'true' && (
                                    <CheckIcon className="h-7 w-7 inline-block text-red-600"/>
                                )
                            }</td>
                        ))}
                    <td>
                </td>
            </tr>
        </tbody>
    </table>
  )
}

export default Table