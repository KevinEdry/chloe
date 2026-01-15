'use client'

import { useState } from 'react'
import Image from 'next/image'

interface Step {
  number: number
  title: string
  description: string
  gif: string
}

const steps: Step[] = [
  {
    number: 1,
    title: 'Install',
    description: 'One command, no dependencies. Works on macOS, Linux, and Windows.',
    gif: '/tapes/step1-install.gif',
  },
  {
    number: 2,
    title: 'Launch',
    description: 'Run chloe in any project directory to start managing your workflow.',
    gif: '/tapes/step2-launch.gif',
  },
  {
    number: 3,
    title: 'Work',
    description: 'Create tasks, launch AI agents in parallel, and ship faster.',
    gif: '/tapes/step3-work.gif',
  },
]

export function HowItWorks() {
  const [activeStep, setActiveStep] = useState(1)
  const currentStep = steps.find((s) => s.number === activeStep) || steps[0]

  return (
    <section className="relative bg-[var(--color-surface)]/30 border-y border-[var(--color-border)]">
      <div className="max-w-[1208px] mx-auto px-6 py-20">
        {/* Header */}
        <div className="max-w-3xl mb-12">
          <h2 className="text-3xl md:text-4xl font-semibold text-[var(--color-text-primary)] mb-4">
            How It Works
          </h2>
          <p className="text-lg text-[var(--color-text-secondary)] leading-relaxed">
            Get up and running in minutes. No complex setup, no configuration files.
          </p>
        </div>

        {/* Main content: Steps on left, Terminal on right */}
        <div className="flex flex-col lg:flex-row gap-8 lg:gap-16">
          {/* Steps - Left side */}
          <div className="lg:w-1/3 space-y-2">
            {steps.map((step) => (
              <div
                key={step.number}
                className={`relative p-4 rounded-xl cursor-pointer transition-all ${
                  activeStep === step.number
                    ? 'bg-[var(--color-primary)]/10 border border-[var(--color-primary)]/20'
                    : 'hover:bg-[var(--color-surface)]/50 border border-transparent'
                }`}
                onMouseEnter={() => setActiveStep(step.number)}
              >
                <div className="flex items-start gap-4">
                  <div
                    className={`flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center text-sm font-bold transition-colors ${
                      activeStep === step.number
                        ? 'bg-[var(--color-primary)] text-white'
                        : 'bg-[var(--color-surface)] text-[var(--color-text-secondary)] border border-[var(--color-border)]'
                    }`}
                  >
                    {step.number}
                  </div>
                  <div>
                    <h3
                      className={`font-semibold mb-1 transition-colors ${
                        activeStep === step.number
                          ? 'text-[var(--color-text-primary)]'
                          : 'text-[var(--color-text-secondary)]'
                      }`}
                    >
                      {step.title}
                    </h3>
                    <p
                      className={`text-sm leading-relaxed transition-colors ${
                        activeStep === step.number
                          ? 'text-[var(--color-text-secondary)]'
                          : 'text-[var(--color-text-tertiary)]'
                      }`}
                    >
                      {step.description}
                    </p>
                  </div>
                </div>
              </div>
            ))}
          </div>

          {/* Terminal GIF - Right side */}
          <div className="lg:w-2/3">
            <div className="rounded-xl overflow-hidden border border-[var(--color-border)] bg-[#0d1117] shadow-2xl">
              <Image
                src={currentStep.gif}
                alt={`Step ${currentStep.number}: ${currentStep.title}`}
                width={800}
                height={400}
                className="w-full h-auto"
                unoptimized
              />
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
