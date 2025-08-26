import { describe, it, expect, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import BaseButton from '@/components/BaseButton.vue'

describe('BaseButton', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('renders properly', () => {
    const wrapper = mount(BaseButton, {
      slots: {
        default: 'Test Button'
      }
    })
    expect(wrapper.text()).toContain('Test Button')
  })

  it('applies correct variant classes', () => {
    const wrapper = mount(BaseButton, {
      props: { variant: 'primary' },
      slots: { default: 'Primary Button' }
    })
    expect(wrapper.classes()).toContain('btn-primary')
  })

  it('applies correct size classes', () => {
    const wrapper = mount(BaseButton, {
      props: { size: 'lg' },
      slots: { default: 'Large Button' }
    })
    expect(wrapper.classes()).toContain('btn-lg')
  })

  it('is disabled when disabled prop is true', () => {
    const wrapper = mount(BaseButton, {
      props: { disabled: true },
      slots: { default: 'Disabled Button' }
    })
    expect(wrapper.element.disabled).toBe(true)
  })

  it('emits click event when clicked', async () => {
    const wrapper = mount(BaseButton, {
      slots: { default: 'Clickable Button' }
    })
    
    await wrapper.trigger('click')
    expect(wrapper.emitted().click).toBeTruthy()
    expect(wrapper.emitted().click.length).toBe(1)
  })
})