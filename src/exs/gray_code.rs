/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   gray_code.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 15:16:35 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 16:28:48 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
 * NOTE: This is a very simple implementation of the Gray code.
 * Documentation used: https://fr.wikipedia.org/wiki/Code_de_Gray (FR 🇫🇷)
 */
pub fn gray_code(n: u32) -> u32 {
	return n ^ (n >> 1);
}
