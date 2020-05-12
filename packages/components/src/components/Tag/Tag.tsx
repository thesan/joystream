import React from "react"
import { makeStyles, TagStyleProps } from "./Tag.styles"

type TagProps = {
  text: string
} & TagStyleProps

export default function Tag({
  text,
  ...styleProps
}: TagProps) {
  let styles = makeStyles(styleProps)
  return (
    <div css={styles}>
      {text}
    </div>
  )
}
