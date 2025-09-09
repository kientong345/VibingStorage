import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import '@/styles/globals.css'

import Header from '@/components/Header'
import Footer from '@/components/Footer'

const inter = Inter({
  subsets: ['latin'],
  variable: '--font-inter',
  weight: ['400', '700'],
  display: 'swap',
})

export const metadata: Metadata = {
  title: 'Blog',
  description: 'A blog about frontend, affiliate, and AI-powered workflows.',
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <body className={`${inter.variable} antialiased`}>
        <Header />
        {children}
        {/* <Footer /> */}
      </body>
    </html>
  )
}
